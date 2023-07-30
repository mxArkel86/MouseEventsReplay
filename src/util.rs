use std::hash::Hash;
use std::{thread};
use enigo::Key;
use keyboard_query::{DeviceState, DeviceQuery};
use mouce::common::MouseEvent;
use std::time::Duration;
use std::collections::HashMap;

pub static mut EVENTS_LIST:Vec<HIDEvent> = Vec::<HIDEvent>::new();
pub(crate) static mut END:bool = false;
pub(crate) static mut POLLING_NUM:f32 = 0.0;

pub fn hold_on_key(device_state:&mut DeviceState){
    let mut keys = Vec::<u16>::new();
    while !keys.contains(&56_u16){
        keys= device_state.get_keys();
        thread::sleep(Duration::from_millis(50));
        //println!("{:?}", keys);
    }
    while keys.contains(&56_u16){
        keys= device_state.get_keys();
        thread::sleep(Duration::from_millis(50));
    }
}



// fn main() {
//     let mut map = std::collections::HashMap::new();

//     map.insert(18 as u16, Key::Layout('1'));
//     map.insert(19 as u16, Key::Layout('2'));
//     map.insert(20 as u16, Key::Layout('3'));
//     map.insert(21 as u16, Key::Layout('4'));
//     map.insert(23 as u16, Key::Layout('5'));
//     map.insert(22 as u16, Key::Layout('6'));
//     map.insert(26 as u16, Key::Layout('7'));
//     map.insert(28 as u16, Key::Layout('8'));
//     map.insert(25 as u16, Key::Layout('9'));
//     map.insert(29 as u16, Key::Layout('0'));
//     map.insert(48 as u16, Key::Tab);
//     map.insert(57 as u16, Key::CapsLock);
//     map.insert(56 as u16, Key::LeftShift);
//     map.insert(33 as u16, Key::LeftBracket);
//     map.insert(30 as u16, Key::RightBracket);
//     map.insert(43 as u16, Key::Comma);
//     map.insert(47 as u16, Key::Period);
//     map.insert(44 as u16, Key::Slash);
//     map.insert(42 as u16, Key::Backslash);
//     map.insert(41 as u16, Key::Semicolon);
//     map.insert(39 as u16, Key::Apostrophe);

//     let enigo = Enigo::new();
    
//     for key in &[18u16, 19u16, 20u16] {
//         enigo.key_click(*map.get(key).unwrap());
//     }
// }


// pub fn getKeyAlphabetHashMap()->HashMap<u16, char>{
//     let mut map = HashMap::new();

//     let keys = [0, 11, 8, 2, 14, 3, 5, 4, 34, 38, 40, 37, 46, 45, 31, 35, 12, 15, 1, 17, 32, 9, 13, 7, 16, 6];
//     let values = "abcdefghijklmnopqrstuvwxyz".chars();

//     for (key, value) in keys.iter().zip(values) {
//         map.insert(*key as u16, value);
//     }
//     return map;
// }

pub enum KEYACTION{
    DOWN,
    UP
}
pub struct KeyEvent{
    pub(crate) action: KEYACTION,
    pub(crate) key: u16
}
pub struct HIDEvent{
    pub(crate) kevt: Option<KeyEvent>,
    pub(crate) mevt: Option<MouseEvent>,
    pub(crate) dt: std::time::Instant
}