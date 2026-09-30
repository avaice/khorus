use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use rodio::{OutputStreamBuilder, Sink};

use crate::keys::Key;
use crate::pack::LoadedPack;

const MAX_OVERLAP: usize = 5;

enum Command {
    Play(Key),
    SetPack(Box<LoadedPack>),
}

#[derive(Clone)]
pub struct AudioEngine {
    commands: Sender<Command>,
}

impl AudioEngine {
    pub fn spawn() -> Self {
        let (commands, receiver) = mpsc::channel();
        thread::spawn(move || run(receiver));
        Self { commands }
    }

    pub fn play(&self, key: Key) {
        let _ = self.commands.send(Command::Play(key));
    }

    pub fn set_pack(&self, pack: LoadedPack) {
        let _ = self.commands.send(Command::SetPack(Box::new(pack)));
    }
}

fn run(receiver: Receiver<Command>) {
    let stream = match OutputStreamBuilder::open_default_stream() {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("音声出力を開けませんでした: {error}");
            return;
        }
    };
    let mut pack: Option<LoadedPack> = None;
    let mut playing: HashMap<usize, VecDeque<Sink>> = HashMap::new();

    for command in receiver {
        match command {
            Command::SetPack(next) => {
                playing.clear();
                pack = Some(*next);
            }
            Command::Play(key) => {
                let Some(pack) = pack.as_ref() else { continue };
                let Some(id) = pack.pick(key) else { continue };
                let Some(sound) = pack.sound(id) else {
                    continue;
                };
                let sinks = playing.entry(id).or_default();
                sinks.retain(|sink| !sink.empty());
                if sinks.len() >= MAX_OVERLAP {
                    if let Some(oldest) = sinks.pop_front() {
                        oldest.stop();
                    }
                }
                let sink = Sink::connect_new(stream.mixer());
                sink.append(sound.clone());
                sinks.push_back(sink);
            }
        }
    }
}
