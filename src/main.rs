mod ingame;

use chrono::prelude::*;
use ingame::quit_from_game;
use std::thread;
use std::time::Duration;
use winput::message_loop::{self, EventReceiver};
use winput::{Action, Vk};

fn read_inputs_from_os(receiver: &EventReceiver) -> Vk {
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == Vk::Escape {
                    println!("Escape pressed");
                } else {
                    return vk;
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let receiver = message_loop::start().unwrap();
    thread::sleep(Duration::from_secs(3));
    // let _ = quit_from_game();
    // let _ = quit_from_main_menu();
    println!("START: {:?}", Utc::now());
    let mut q_count = 0;

    loop {
        let vk = read_inputs_from_os(&receiver);
        if vk == winput::Vk::Q {
            q_count += 1;
            println!("Q count is {:?}", q_count);
        } else {
            println!("KEY: {:?}", vk);
        }
        if q_count == 3 {
            println!("Quitting from game");
            quit_from_game();
            println!("QUIT : {:?}", Utc::now());

            break;
        }
    }
}
