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

pub fn take_screenshot(one_frame: &Duration) -> Result<()> {
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display)?;
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        println!("{:?}\tscreenshot_call.", Utc::now());
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    println!("Missed capture");
                    thread::sleep(*one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
        // Flip the ARGB image into a BGRA image.
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
            }
        }
        // Save the image.
        let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("screenshots/ER {}.png", timestamp);

        repng::encode(File::create(&filename)?, w as u32, h as u32, &bitflipped)?;
        break;
    }
    Ok(())
}
