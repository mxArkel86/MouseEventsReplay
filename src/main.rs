use std::thread;
use mouce::{Mouse, common::MouseEvent, MouseActions};
use std::time::Duration;
use keyboard_query::{DeviceQuery, DeviceState};

const POLLING_PERIOD:u64 = 10;
static mut CONTINUE_RECORDING: bool = true;

static mut EVENTS_LIST:Vec<MouseEvent> = Vec::<MouseEvent>::new();

fn disable_event_hooks(mouse_manager:&mut Box<dyn MouseActions>){
    let _ = mouse_manager.unhook_all();
}
fn enable_event_hooks(mouse_manager:&mut Box<dyn MouseActions>){
    let _ = mouse_manager.hook(Box::new(|e| {
        //println!("New event: {:?}", e);
        unsafe{
        match e{
            MouseEvent::RelativeMove(_xpos,_ypos) => {},
            MouseEvent::AbsoluteMove(_xpos, _ypos) => {
                    if CONTINUE_RECORDING{
                        EVENTS_LIST.push(*e);
                    }
            },
            MouseEvent::Press(_btn) => {
                if CONTINUE_RECORDING{
                    EVENTS_LIST.push(*e);
                }
            },
            MouseEvent::Release(_btn) => {
                if CONTINUE_RECORDING{
                    EVENTS_LIST.push(*e);
                }
            },
            MouseEvent::Scroll(_direction) => {}
        }
    }
    }));
}
fn hold_on_key(device_state:&DeviceState){
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
fn main(){
    unsafe{
    //initialize
    let mut device_state = DeviceState::new();
    let mut mouse_manager: Box<dyn MouseActions> = Mouse::new();
    
    //hold
    println!("waiting to begin recording...");

    hold_on_key(&mut device_state);

    println!("recording started");

    //begin recording mouse inputs
    enable_event_hooks(&mut mouse_manager);
    
    hold_on_key(&mut device_state);
    //wait for shift key to be pressed again
    disable_event_hooks(&mut mouse_manager);


    println!("actions={}", EVENTS_LIST.len());

    println!("waiting to replay events");
    hold_on_key(&mut device_state);

    for i in 0..EVENTS_LIST.len(){
        let event = EVENTS_LIST[i];
        thread::sleep(Duration::from_millis(10));
        match event{
            MouseEvent::RelativeMove(_xpos, _ypos) => {},
            MouseEvent::AbsoluteMove(xpos, ypos) => {
                let _  = mouse_manager.move_to(xpos as usize, ypos as usize);
            },
            MouseEvent::Release(btn) => {
                match btn {
                    mouce::common::MouseButton::Left => {
                        let _  = mouse_manager.press_button(&mouce::common::MouseButton::Left);
                    },
                    mouce::common::MouseButton::Middle => {
                        let _  = mouse_manager.press_button(&mouce::common::MouseButton::Middle);
                    },
                    mouce::common::MouseButton::Right => {
                        let _  = mouse_manager.press_button(&mouce::common::MouseButton::Right);
                    },
                }
            },
            MouseEvent::Scroll(_direction) => {
                // match  direction {
                //     mouce::common::ScrollDirection::Up => {
                //         let _  = mouse_manager.scroll_wheel(&mouce::common::ScrollDirection::Up);
                //     },
                //     mouce::common::ScrollDirection::Down => {
                //         let _  = mouse_manager.scroll_wheel(&mouce::common::ScrollDirection::Down);
                //     },
                //     mouce::common::ScrollDirection::Right => {
                //         let _  = mouse_manager.scroll_wheel(&mouce::common::ScrollDirection::Right);
                //     },
                //     mouce::common::ScrollDirection::Left => {
                //         let _  = mouse_manager.scroll_wheel(&mouce::common::ScrollDirection::Left);
                //     },
                // }
            }
            MouseEvent::Press(btn) => {
                match btn {
                    mouce::common::MouseButton::Left => {
                        let _  = mouse_manager.press_button(&mouce::common::MouseButton::Left);
                    },
                    mouce::common::MouseButton::Middle => {
                        let _  = mouse_manager.press_button(&mouce::common::MouseButton::Middle);
                    },
                    mouce::common::MouseButton::Right => {
                        let _  = mouse_manager.press_button(&mouce::common::MouseButton::Right);
                    },
                }
            }
        }
    }
}
}
