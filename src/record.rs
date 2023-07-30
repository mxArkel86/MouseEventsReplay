use std::thread::{ self, JoinHandle };

use crate::util::{
    EVENTS_LIST,
    RIGHT_SHIFT_PRESSED,
    KEY_UP_POLLING,
    POLLING_UP_PRESSED,
    KEY_DOWN_POLLING,
    POLLING_DOWN_PRESSED,
};
static mut DO_LISTEN: bool = false;

use rdev::{ self, Event, EventType, Key };

fn callback(event: Event) {
    unsafe {
        match event.event_type {
            EventType::KeyPress(key) => {
                if key == Key::ShiftRight {
                    RIGHT_SHIFT_PRESSED = true;
                    return;
                }
                if key == KEY_UP_POLLING {
                    POLLING_UP_PRESSED = true;
                }
                if key == KEY_DOWN_POLLING {
                    POLLING_DOWN_PRESSED = true;
                }
                if DO_LISTEN {
                    EVENTS_LIST.push(event.event_type);
                }
            }
            EventType::KeyRelease(key) => {
                if key == Key::ShiftRight {
                    RIGHT_SHIFT_PRESSED = false;
                    return;
                }
                if key == KEY_UP_POLLING {
                    POLLING_UP_PRESSED = false;
                }
                if key == KEY_DOWN_POLLING {
                    POLLING_DOWN_PRESSED = false;
                }
                if DO_LISTEN {
                    EVENTS_LIST.push(event.event_type);
                }
            }
            EventType::ButtonPress(_btn) => {
                if DO_LISTEN {
                    EVENTS_LIST.push(event.event_type);
                }
            }
            EventType::ButtonRelease(_btn) => {
                if DO_LISTEN {
                    EVENTS_LIST.push(event.event_type);
                }
            }
            EventType::MouseMove { x: _, y: _ } => {
                if DO_LISTEN {
                    EVENTS_LIST.push(event.event_type);
                }
            }
            EventType::Wheel { delta_x: _, delta_y: _ } => {
                if DO_LISTEN {
                    EVENTS_LIST.push(event.event_type);
                }
            }
        }
    }
}

pub fn initialize_input_listener() -> JoinHandle<()> {
    let handle = thread::spawn(move || {
        let resp = rdev::listen(callback);
        match resp {
            Ok(_o) => { println!("listener initialized. ok.") }
            Err(e) => {
                println!("error {:?}", e);
            }
        }
    });
    return handle;
}

pub fn record() {
    unsafe {
        DO_LISTEN = true;
    }
}

pub fn endrecord() {
    unsafe {
        DO_LISTEN = false;
    }
}
