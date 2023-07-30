use std::{thread::{self, JoinHandle}, time::Duration};

use keyboard_query::{DeviceState, DeviceQuery};
use mouce::{common::MouseEvent, MouseActions};

use crate::util::{HIDEvent, KeyEvent, KEYACTION, EVENTS_LIST};
static mut RECORDING:bool = false;

const EXCLUDE_KEYS:[u16; 3] = [56, 123, 124];


pub fn record(mouse_manager:&mut Box<dyn MouseActions>)->JoinHandle<()>{
    unsafe{
        
    RECORDING = true;

    EVENTS_LIST.clear();

    enable_mouse_event_hooks(mouse_manager);

    let handle = enable_key_event_polling();
    
    return handle;
    }
}

pub fn endrecord(mouse_manager:&mut Box<dyn MouseActions>){
    unsafe{
        let _ = mouse_manager.unhook_all();

        RECORDING = false;
    }
}

fn enable_key_event_polling()->JoinHandle<()>{
    unsafe{


    let handle = thread::spawn(move || {
        let mut keys;
        let mut prevkeys = Vec::<u16>::new();
        let device_state = DeviceState::new();
        
        while RECORDING{
            keys= device_state.get_keys();
            let keys_down:Vec<u16> = keys.iter().filter(|&x|!prevkeys.contains(&x)).cloned().collect();
            let keys_up:Vec<u16> = prevkeys.iter().filter(|&x|!keys.contains(&x)).cloned().collect();
            
            //let mut evt_vec = ev.lock().unwrap();
            
            let time_now = std::time::Instant::now();
            for key in keys_down{
                if EXCLUDE_KEYS.contains(&key){
                    continue;
                }
                let evt:HIDEvent = HIDEvent{
                    kevt: Some(KeyEvent{
                        action: KEYACTION::DOWN,
                        key: key,
                    }),
                    mevt: None,
                    dt: time_now
                };
                EVENTS_LIST.push(evt);
            }
            for key in keys_up{
                if EXCLUDE_KEYS.contains(&key){
                    continue;
                }
                let evt:HIDEvent = HIDEvent{
                    kevt: Some(KeyEvent{
                        action: KEYACTION::UP,
                        key: key,
                    }),
                    mevt: None,
                    dt: time_now
                };
                EVENTS_LIST.push(evt);
            }
            prevkeys = keys;

            thread::sleep(Duration::from_millis(1));
        }
    });
    
    return handle;
}
}

fn enable_mouse_event_hooks(mouse_manager:&mut Box<dyn MouseActions>){
    unsafe{
    let _ = mouse_manager.hook(Box::new(|e| {
        //println!("New event: {:?}", e);
        
        let evt:HIDEvent = HIDEvent{
            kevt: None,
            mevt: Some(*e),
            dt: std::time::Instant::now()
        };
        match e{
            MouseEvent::RelativeMove(_xpos,_ypos) => {},
            MouseEvent::AbsoluteMove(_xpos, _ypos) => {
                EVENTS_LIST.push(evt);
            },
            MouseEvent::Press(_btn) => {
                EVENTS_LIST.push(evt);
            },
            MouseEvent::Release(_btn) => {
                EVENTS_LIST.push(evt);
            },
            MouseEvent::Scroll(_direction) => {}
        }
        
    
    }));
}
}