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

pub fn check_quit_call(
    receiver: &EventReceiver,
    verbose: bool,
    q_count: &mut usize,
) -> Result<usize> {
    println!("{:?}\tChecking for quitcalls.", Utc::now());

    // loop {
    let vk = read_inputs_from_os(&receiver, verbose);
    if vk == winput::Vk::J {
        *q_count += 1;
        println!("Q count is {:?}", q_count);
    }
    if *q_count == 3 {
        println!("{:?}\tQuitcall.", Utc::now());
        crate::ingame::quit_from_game();
        println!("{:?}\tQuit.", Utc::now());
        return Ok(0); // reset the counter
    }
    if vk == winput::Vk::X {
        panic!("X pressed");
    } else {
        println!("KEY: {:?}", vk);
    }
    Ok(*q_count)
}

pub fn check_sysreq(verbose: bool) {
    println!("{:?}\tChecking for printscreen.", Utc::now());
    let receiver = message_loop::start().unwrap();
    let mut q_count = 0;
    let mut enigo = Enigo::new();

    loop {
        let vk = read_inputs_from_os(&receiver, verbose);
        if vk == winput::Vk::J {
            q_count += 1;
            println!("Q count is {:?}", q_count);
        } else {
            println!("KEY: {:?}", vk);
        }
        if q_count == 3 {
            println!("{:?}\tprintcall.", Utc::now());
            enigo.key_down(Key::Meta);
            enigo.key_click(Key::Raw(44));
            enigo.key_up(Key::Meta);
            println!("{:?}\tsaved.", Utc::now());
            break;
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

pub fn file_exists(path: &str) -> bool {
    // see if a .png exists in this dir that was created recently
    let file = File::open(path);
    match file {
        Ok(_) => {
            println!("{:?}\tFile exists.", Utc::now());
            true
        }
        Err(e) => {
            println!("{:?}\tFile problem: {:?}.", Utc::now(), e);
            false
        }
    }
}

pub fn check_screenshot(receiver: &EventReceiver) -> Result<String> {
    let vk = read_inputs_from_os(&receiver, true);
    if vk != winput::Vk::L {
        return Ok("".to_string());
    }
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        println!("{:?}\tscreenshot_call.", Utc::now());
        // Wait until there's a frame.
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    println!("Missed capture");
                    thread::sleep(one_frame);
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

        return Ok(filename);
    }
}
