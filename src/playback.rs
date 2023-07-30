use std::{thread::{self, JoinHandle}, time::Duration, f32::consts::E};

use crate::util::{EVENTS_LIST, POLLING_NUM, END};
use enigo::{self, Enigo, KeyboardControllable, MouseControllable};


pub fn beginPlayback()->JoinHandle<()>{
    
    unsafe{
    let handle = thread::spawn(move|| {
        let mut enigo_ = Enigo::new();
        while !END {
            let polling_period = 10.0_f32*E.powf(-POLLING_NUM);
            
            for i in 0..EVENTS_LIST.len(){
                if END{
                    break;
                }
                let event = EVENTS_LIST.get(i).unwrap();
                thread::sleep(Duration::from_millis(polling_period as u64));
    
                if event.kevt.is_some(){
                    let mut s = String::from("");
                    s.insert(0, char::from_digit(event.kevt.as_ref().unwrap().key as u32, 10).unwrap());
                    
                }else if(event.mevt.is_some()){
                    match event.mevt.unwrap(){
                        mouce::common::MouseEvent::RelativeMove(xpos, ypos) => {
                            enigo_.mouse_move_relative(xpos, ypos);
                        },
                        mouce::common::MouseEvent::AbsoluteMove(xpos, ypos) => {
                            enigo_.mouse_move_to(xpos, ypos);
                        },
                        mouce::common::MouseEvent::Press(button) => {
                            match button{
                                mouce::common::MouseButton::Left => enigo_.mouse_down(enigo::MouseButton::Left),
                                mouce::common::MouseButton::Middle => enigo_.mouse_down(enigo::MouseButton::Middle),
                                mouce::common::MouseButton::Right => enigo_.mouse_down(enigo::MouseButton::Right),
                            }
                        },
                        mouce::common::MouseEvent::Release(button) => {
                            match button{
                                mouce::common::MouseButton::Left => enigo_.mouse_up(enigo::MouseButton::Left),
                                mouce::common::MouseButton::Middle => enigo_.mouse_up(enigo::MouseButton::Middle),
                                mouce::common::MouseButton::Right => enigo_.mouse_up(enigo::MouseButton::Right),
                            }
                        },
                        mouce::common::MouseEvent::Scroll(direction) => {
                            match direction{
                                mouce::common::ScrollDirection::Up => enigo_.mouse_scroll_y(1),
                                mouce::common::ScrollDirection::Down => enigo_.mouse_scroll_y(-1),
                                mouce::common::ScrollDirection::Right => enigo_.mouse_scroll_x(1),
                                mouce::common::ScrollDirection::Left => enigo_.mouse_scroll_x(-1),
                            }
                        },
                    }
                }
            }
        }
    });
    return handle;
}
}