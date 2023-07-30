use std::thread;
use std::time::Duration;
mod util;
use crate::playback::begin_playback;
use crate::util::{
    hold_on_key,
    EVENTS_LIST,
    POLLING_NUM,
    END,
    RIGHT_SHIFT_PRESSED,
    POLLING_UP_PRESSED,
    POLLING_CHNG,
    POLLING_DOWN_PRESSED,
};
mod record;
use crate::record::{ record, endrecord, initialize_input_listener };
mod playback;

fn main() {
    unsafe {
        println!("starting program");
        let _handle = initialize_input_listener();

        //hold
        println!("waiting to begin recording...");

        hold_on_key();

        println!("recording started");

        //begin recording mouse inputs
        record();

        hold_on_key();
        //wait for shift key to be pressed again
        endrecord();

        println!("actions={}", EVENTS_LIST.len());

        println!("waiting to replay events");
        hold_on_key();

        begin_playback();

        loop {
            if RIGHT_SHIFT_PRESSED {
                END = true;
                break;
            }
            if POLLING_UP_PRESSED && !POLLING_DOWN_PRESSED {
                POLLING_NUM += POLLING_CHNG;
            }
            if POLLING_DOWN_PRESSED && !POLLING_UP_PRESSED {
                POLLING_NUM -= POLLING_CHNG;
            }
            print!("\rpolling num={:.8}", POLLING_NUM);
            thread::sleep(Duration::from_millis(10));
        }

        // handle.join().unwrap();
        print!("\n");
        println!("program ended");
    }
}
