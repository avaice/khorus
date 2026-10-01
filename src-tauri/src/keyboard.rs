use std::ffi::c_ulong;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use core_foundation::runloop::{kCFRunLoopCommonModes, kCFRunLoopDefaultMode, CFRunLoop};
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
    CallbackResult, EventField,
};
use foreign_types::ForeignType;

use crate::keys::Key;

const KEYCODE_SPACE: i64 = 49;
const KEYCODE_RETURN: i64 = 36;
const KEYCODE_KEYPAD_ENTER: i64 = 76;
const KEYCODE_DELETE: i64 = 51;
const RETRY_INTERVAL: Duration = Duration::from_secs(1);
const RUN_LOOP_SLICE: Duration = Duration::from_millis(500);

extern "C" {
    fn CGPreflightListenEventAccess() -> bool;
    fn CGRequestListenEventAccess() -> bool;
    fn CGEventKeyboardGetUnicodeString(
        event: *mut core::ffi::c_void,
        max_length: c_ulong,
        actual_length: *mut c_ulong,
        buffer: *mut u16,
    );
}

pub fn has_permission() -> bool {
    unsafe { CGPreflightListenEventAccess() }
}

pub fn request_permission() -> bool {
    unsafe { CGRequestListenEventAccess() }
}

pub fn spawn(on_key: impl Fn(Key, bool) + Send + Sync + 'static) {
    let on_key = Arc::new(on_key);
    thread::spawn(move || loop {
        let tap_disabled = Arc::new(AtomicBool::new(false));
        let callback_on_key = Arc::clone(&on_key);
        let callback_disabled = Arc::clone(&tap_disabled);
        let tap = CGEventTap::new(
            CGEventTapLocation::Session,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::ListenOnly,
            vec![CGEventType::KeyDown],
            move |_proxy, event_type, event| {
                match event_type {
                    CGEventType::KeyDown => {
                        let is_repeat = event
                            .get_integer_value_field(EventField::KEYBOARD_EVENT_AUTOREPEAT)
                            != 0;
                        if let Some(key) = key_from_event(event) {
                            callback_on_key(key, is_repeat);
                        }
                    }
                    CGEventType::TapDisabledByTimeout | CGEventType::TapDisabledByUserInput => {
                        callback_disabled.store(true, Ordering::SeqCst);
                    }
                    _ => {}
                }
                CallbackResult::Keep
            },
        );
        let Ok(tap) = tap else {
            thread::sleep(RETRY_INTERVAL);
            continue;
        };
        let Ok(source) = tap.mach_port().create_runloop_source(0) else {
            thread::sleep(RETRY_INTERVAL);
            continue;
        };
        CFRunLoop::get_current().add_source(&source, unsafe { kCFRunLoopCommonModes });
        tap.enable();
        loop {
            CFRunLoop::run_in_mode(unsafe { kCFRunLoopDefaultMode }, RUN_LOOP_SLICE, false);
            if tap_disabled.swap(false, Ordering::SeqCst) {
                tap.enable();
            }
        }
    });
}

fn key_from_event(event: &CGEvent) -> Option<Key> {
    match event.get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) {
        KEYCODE_SPACE => Some(Key::Space),
        KEYCODE_RETURN | KEYCODE_KEYPAD_ENTER => Some(Key::Enter),
        KEYCODE_DELETE => Some(Key::Backspace),
        _ => char_from_event(event).map(Key::Char),
    }
}

fn char_from_event(event: &CGEvent) -> Option<char> {
    let mut buffer = [0u16; 4];
    let mut length: c_ulong = 0;
    unsafe {
        CGEventKeyboardGetUnicodeString(
            event.as_ptr().cast(),
            buffer.len() as c_ulong,
            &mut length,
            buffer.as_mut_ptr(),
        );
    }
    let written = &buffer[..(length as usize).min(buffer.len())];
    let c = char::decode_utf16(written.iter().copied()).next()?.ok()?;
    if c.is_control() {
        return None;
    }
    Some(c.to_lowercase().next().unwrap_or(c))
}
