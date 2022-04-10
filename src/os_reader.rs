use anyhow::{anyhow, Result};
use chrono::prelude::*;
use enigo::*;
use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;
use winput::message_loop::{self, EventReceiver};
use winput::{Action, Vk};

pub fn read_inputs_from_os(receiver: &EventReceiver, verbose: bool) -> Vk {
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if verbose {
                    println!("{:?}: {:^80}", vk, Utc::now().format("%H:%M:%S"));
                }
                return vk;
            }
            _ => (),
        }
    }
}

pub fn check_monitors() {
    let displays = Display::all().expect("Unable to index displays");

    for (i, display) in displays.iter().enumerate() {
        println!(
            "Display {} [{}x{}]",
            i + 1,
            display.width(),
            display.height()
        );
    }
}
