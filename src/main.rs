use std::io::{stdout, Write, stdin};
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


fn getInput(prompt:&str)->String{
    

    let mut line = String::new();
    println!("{}",prompt);
    let _=stdout().flush();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    
    return line.trim().to_string();
}
fn strToInt(str:String)->i32{
    return str.parse::<i32>().unwrap();
}
fn strToFloat(str:String)->f32{
    return str.parse::<f32>().unwrap();
}

fn main() {
    let mut repetitions: i32 = 999999;
    let mut s_betw_rep:f32 = 0.0;

    unsafe {
        println!("starting program");
        let _handle = initialize_input_listener();
        println!("press right shift to perform basic actions. press right arrow or left arrow during playback to change the speed.");
        let line = getInput("how many time should this be run? (default = infinity)");
        if line!="0"{
        repetitions = strToInt(line);
        }
        
        let line = getInput("seconds between repetitions?");
        if line!="0"{
            s_betw_rep = strToFloat(line);
        }

        //hold
        println!("waiting to begin recording... (press right shift)");

        hold_on_key();

        println!("recording started (press right shift to end)");

        //begin recording mouse inputs
        record();

        hold_on_key();
        //wait for shift key to be pressed again
        endrecord();

        println!("# of actions={}", EVENTS_LIST.len());

        println!("waiting to replay events");
        hold_on_key();
        println!("press right shift to end");

        let handle = begin_playback(repetitions, s_betw_rep);

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
            thread::sleep(Duration::from_millis(10));
        }

        handle.join().unwrap();
        print!("\n");
        println!("program ended");
    }
}
