use std::{ thread::{ self, JoinHandle }, time::Duration, f32::consts::E, io::{stdout, Write} };
use crate::util::{ EVENTS_LIST, POLLING_NUM, END, is_letter, LETTER_PERIOD_MULT };

pub fn begin_playback(repetitions:i32, s_betw_rep:f32) -> JoinHandle<()> {
    unsafe {
        let handle = thread::spawn(move || {
            
            let mut n = 0;
            
            print!("...");

            while !END && n<repetitions {
                n+=1;
                
                let secs = s_betw_rep;
                print!("\rWaiting {} seconds", secs);
                let _=stdout().flush();
                thread::sleep(Duration::from_secs_f32(secs));
                

                let polling_period = 10.0_f32 * E.powf(-POLLING_NUM);

                for i in 0..EVENTS_LIST.len() {
                    if END {
                        break;
                    }
                    let event = EVENTS_LIST.get(i).unwrap();
                    match event {
                        rdev::EventType::KeyPress(key) => {
                            if is_letter(key) {
                                thread::sleep(
                                    Duration::from_millis(
                                        (polling_period * (LETTER_PERIOD_MULT - 1_f32)) as u64
                                    )
                                );
                            }
                            let _ = rdev::simulate(event);
                        }
                        rdev::EventType::KeyRelease(key) => {
                            if is_letter(key) {
                                thread::sleep(
                                    Duration::from_millis(
                                        (polling_period * (LETTER_PERIOD_MULT - 1_f32)) as u64
                                    )
                                );
                            }
                            let _ = rdev::simulate(event);
                        }
                        rdev::EventType::ButtonPress(_) => {
                            let _ = rdev::simulate(event);
                        }
                        rdev::EventType::ButtonRelease(_) => {
                            let _ = rdev::simulate(event);
                        }
                        rdev::EventType::MouseMove { x: _, y: _ } => {
                            let _ = rdev::simulate(event);
                        }
                        rdev::EventType::Wheel { delta_x: _, delta_y: _ } => {
                            let _ = rdev::simulate(event);
                        }
                    }
                    thread::sleep(Duration::from_millis(polling_period as u64));
                }
            }
        });
        return handle;
    }
}
