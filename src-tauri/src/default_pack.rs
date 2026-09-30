use crate::pack::{self, LoadedPack, PackError, PackFiles};

pub fn load() -> Result<LoadedPack, PackError> {
    let files: PackFiles = [(
        "pack.json".to_string(),
        include_bytes!("../default-pack/pack.json").to_vec(),
    )]
    .into_iter()
    .collect();
    pack::load(&files)
}
