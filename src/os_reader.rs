use chrono::prelude::*;
use winput::message_loop::{self, EventReceiver};
use winput::{Action, Vk};

fn read_inputs_from_os(receiver: &EventReceiver, verbose: bool) -> Vk {
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == Vk::Escape && verbose {
                    println!("Escape pressed");
                } else {
                    return vk;
                }
            }
            _ => (),
        }
    }
}

pub fn check_quit_call() {
    println!("Checking for quitcalls: {:?}", Utc::now());
    let receiver = message_loop::start().unwrap();
    let mut q_count = 0;

    loop {
        let vk = read_inputs_from_os(&receiver);
        if vk == winput::Vk::J {
            q_count += 1;
            println!("Q count is {:?}", q_count);
        } else {
            println!("KEY: {:?}", vk);
        }
        if q_count == 3 {
            println!("Quitting from game");
            crate::ingame::quit_from_game();
            println!("QUIT : {:?}", Utc::now());

            break;
        }
    }
}
