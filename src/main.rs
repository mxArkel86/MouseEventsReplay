
use std::cmp::Ordering;
use std::thread::JoinHandle;
use std::thread;
use std::time::Duration;
mod util;
use keyboard_query::{DeviceState, DeviceQuery};
use mouce::{Mouse, MouseActions};
use crate::playback::beginPlayback;
use crate::util::{hold_on_key, EVENTS_LIST, POLLING_NUM, END};
mod record;
use crate::record::{record, endrecord};
mod playback;

use rdev::{listen, Event};


fn main(){
    unsafe{
        let mut device_state: DeviceState = DeviceState::new();
        let mut mouse_manager: Box<dyn MouseActions> = Mouse::new();
        
        

        
    
        // let mut newkeys = Vec::<u16>::new();
        // let mut keys = Vec::<u16>::new();
        // loop{
        //     keys= device_state.get_keys();
        //     if keys.len()>0{
        //     let key1 = keys.get(0).unwrap();
        //     if !newkeys.contains(key1)
        //     {
        //         newkeys.push(*key1);
        //     }
        //      }
        //     println!("{:?}", newkeys);
            
        //     thread::sleep(Duration::from_millis(10));
        // }

    //hold
    println!("waiting to begin recording...");

    hold_on_key(&mut device_state);

    println!("recording started");

    //begin recording mouse inputs
    let handle: JoinHandle<()> = record(&mut mouse_manager);
    
    hold_on_key(&mut device_state);
    //wait for shift key to be pressed again
    endrecord(&mut mouse_manager);
    handle.join().unwrap();

    EVENTS_LIST.sort_by(|a, b| a.dt.cmp(&b.dt));

    println!("actions={}", EVENTS_LIST.len());
    
    println!("waiting to replay events");
    hold_on_key(&mut device_state);

    beginPlayback();


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
        thread::sleep(Duration::from_millis(10));
    }

    // handle.join().unwrap();

   println!("program ended");

    }
}
