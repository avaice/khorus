# Sound Pack Specification

## Format

- A zip archive
- Contains pack.json (the key map) and the audio files at its root

## pack.json

- title: Name of the pack (required)
- description: Description or credits (optional)
- keys: Object mapping key names to sound specifiers (required)
  - A key name is a single lowercase character, or space, enter, or backspace
  - A sound specifier is a path relative to the pack, or macos:name
- fallback: Array of sound specifiers played for character keys not listed in keys (optional)

## Metadata

- description can contain free-form text such as credits
- title and description are shown in the list and detail views after import

## Specifying Audio Files

- Files in the pack are specified by paths relative to the pack root
- References to parent directories and symbolic links in the archive are rejected
- To use built-in OS sounds, specify a name prefixed with the OS name
  - Example: macos:Tink
  - macos: resolves only to files under /System/Library/Sounds
  - Absolute paths are not allowed
- If the specified name does not exist in the environment, that sound is not played

## Key Map Behavior

- Sounds play only on key press, not on key release
- Key repeat from holding a key down does not play sounds. Only the initial press does
- Character keys are assigned sounds by the character they input (lowercased)
- Character keys without an assignment (numbers, symbols, etc.) play a sound chosen at random from the fallback pool
- Special keys (space, enter, backspace, etc.) can be assigned individual sounds
- Special keys without an assignment (Shift, arrow keys, etc.) play no sound
- The same sound can overlap up to 5 times. Beyond that, the oldest playback is stopped and the sound is played again

## Import and Management

- A zip chosen by the user is validated and, if valid, saved to the app's data directory
- Imported packs are added to the list, can be selected to switch between them, and can be deleted when no longer needed
- The app can include multiple built-in packs. Built-in packs cannot be deleted
- The selected pack persists across launches. If it cannot be loaded, the first built-in pack is used
- Only zips with pack.json at the root are accepted
- Metadata added by macOS (__MACOSX, .DS_Store, etc.) is ignored

## Validation

- File size, sound duration, and audio format are validated on import
- Limits are 5MB per file, 10 seconds per sound, 50MB total after extraction, and 500 files
- Supported audio formats are mp3, wav, flac, ogg, aiff, and m4a
- Packs that exceed the limits or use unsupported formats are not imported

## Open Questions

- Adding names for more special keys (arrow keys, Tab, Esc, etc.)
- Handling zips that contain a single folder
