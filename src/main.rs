use std::{thread, f32::consts::E};
use mouce::{Mouse, common::MouseEvent, MouseActions};
use std::time::Duration;
use keyboard_query::{DeviceQuery, DeviceState};

static mut POLLING_NUM:f32 = 0.0;
static mut CONTINUE_RECORDING: bool = true;
static mut END:bool = false;

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
            MouseEvent::Scroll(_direction) => {
                if CONTINUE_RECORDING{
                    EVENTS_LIST.push(*e);
                }
            }
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
    
    let handle = thread::spawn(|| {
        let mouse_m: Box<dyn MouseActions> = Mouse::new();
        while !END {
            let polling_period = 10.0_f32*E.powf(-POLLING_NUM);
            for i in 0..EVENTS_LIST.len(){
                if END{
                    break;
                }
                let event = EVENTS_LIST[i];
                thread::sleep(Duration::from_millis(polling_period as u64));
    
                match event{
                    MouseEvent::RelativeMove(_xpos, _ypos) => {},
                    MouseEvent::AbsoluteMove(xpos, ypos) => {
                        let _  = mouse_m.move_to(xpos as usize, ypos as usize);
                    },
                    MouseEvent::Release(btn) => {
                        match btn {
                            mouce::common::MouseButton::Left => {
                                let _  = mouse_m.press_button(&mouce::common::MouseButton::Left);
                            },
                            mouce::common::MouseButton::Middle => {
                                let _  = mouse_m.press_button(&mouce::common::MouseButton::Middle);
                            },
                            mouce::common::MouseButton::Right => {
                                let _  = mouse_m.press_button(&mouce::common::MouseButton::Right);
                            },
                        }
                    },
                    MouseEvent::Scroll(direction) => {
                        match direction {
                            mouce::common::ScrollDirection::Up => {
                                let _  = mouse_m.scroll_wheel(&mouce::common::ScrollDirection::Up);
                            },
                            mouce::common::ScrollDirection::Down => {
                                let _  = mouse_m.scroll_wheel(&mouce::common::ScrollDirection::Down);
                            },
                            mouce::common::ScrollDirection::Right => {
                                let _  = mouse_m.scroll_wheel(&mouce::common::ScrollDirection::Right);
                            },
                            mouce::common::ScrollDirection::Left => {
                                let _  = mouse_m.scroll_wheel(&mouce::common::ScrollDirection::Left);
                            },
                        }
                    }
                    MouseEvent::Press(btn) => {
                        match btn {
                            mouce::common::MouseButton::Left => {
                                let _  = mouse_m.press_button(&mouce::common::MouseButton::Left);
                            },
                            mouce::common::MouseButton::Middle => {
                                let _  = mouse_m.press_button(&mouce::common::MouseButton::Middle);
                            },
                            mouce::common::MouseButton::Right => {
                                let _  = mouse_m.press_button(&mouce::common::MouseButton::Right);
                            },
                        }
                    }
                }
            }
        }
    });

    let mut keys = Vec::<u16>::new();
    loop{
        keys= device_state.get_keys();
        if keys.contains(&56_u16){
            END = true;
            break;
        }
        if keys.contains(&124_u16){
            POLLING_NUM+=0.01;
        }
        if keys.contains(&123_u16){
            POLLING_NUM-=0.01;
        }
        print!("\rpolling num={:.8}", POLLING_NUM);
        thread::sleep(Duration::from_millis(20));
    }

    handle.join().unwrap();

    println!("program ended");

    
}
}
