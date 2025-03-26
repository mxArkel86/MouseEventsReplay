use rdev::{ EventType, Key };
use std::{ time::Duration, thread };

pub static mut EVENTS_LIST: Vec<EventType> = Vec::<EventType>::new();
pub static mut END: bool = false;
pub static mut POLLING_NUM: f32 = 0.0;
pub const POLLING_CHNG: f32 = 0.004;
pub const LETTER_PERIOD_MULT: f32 = 4.0;
pub static mut RIGHT_SHIFT_PRESSED: bool = false;
pub const KEY_DOWN_POLLING: Key = Key::LeftArrow;
pub const KEY_UP_POLLING: Key = Key::RightArrow;
pub static mut POLLING_UP_PRESSED: bool = false;
pub static mut POLLING_DOWN_PRESSED: bool = false;

pub fn hold_on_key() {
    unsafe {
        while !RIGHT_SHIFT_PRESSED {
            thread::sleep(Duration::from_millis(50));
        }
        while RIGHT_SHIFT_PRESSED {
            thread::sleep(Duration::from_millis(50));
        }
    }
}

const ALL_LETTERS: [Key; 26] = [
    Key::KeyA,
    Key::KeyB,
    Key::KeyC,
    Key::KeyD,
    Key::KeyE,
    Key::KeyF,
    Key::KeyG,
    Key::KeyH,
    Key::KeyI,
    Key::KeyJ,
    Key::KeyK,
    Key::KeyL,
    Key::KeyM,
    Key::KeyN,
    Key::KeyO,
    Key::KeyP,
    Key::KeyQ,
    Key::KeyR,
    Key::KeyS,
    Key::KeyT,
    Key::KeyU,
    Key::KeyV,
    Key::KeyW,
    Key::KeyX,
    Key::KeyY,
    Key::KeyZ,
];
pub fn is_letter(key: &Key) -> bool {
    return ALL_LETTERS.contains(key);
}
