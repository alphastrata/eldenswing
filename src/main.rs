use enigo::*;

use std::thread;
use std::time::Duration;

struct QuitButton {
    x: i32,
    y: i32,
}

impl QuitButton {
    fn new(x: i32, y: i32) -> Self {
        QuitButton { x, y }
    }
}

fn main() {
    let mut enigo = Enigo::new();
    let wait_time = Duration::from_secs(4);
    let refrate = 120; // game tickrate should be around 1/59.9995ths of a second

    thread::sleep(wait_time);
    enigo.key_click(Key::Escape);

    thread::sleep(Duration::from_millis(refrate * 3));
    enigo.key_click(Key::UpArrow);

    thread::sleep(Duration::from_millis(refrate * 6));
    enigo.key_click(Key::Layout('e'));

    thread::sleep(Duration::from_millis(refrate * 3));
    let quit = QuitButton::new(4002, 280); // RHSMonitor when in 1440p

    thread::sleep(Duration::from_millis(refrate * 2));
    enigo.mouse_move_to(quit.x, quit.y);
    thread::sleep(Duration::from_millis(refrate * 2));

    enigo.key_click(Key::Layout('e'));
    thread::sleep(Duration::from_millis(refrate * 2));
    println!("on button, just hit e");

    enigo.key_click(Key::LeftArrow);
    thread::sleep(Duration::from_millis(refrate * 2));
    println!("should be on Y");

    enigo.key_click(Key::Layout('e'));
    thread::sleep(Duration::from_millis(refrate * 2));
    println!("done");
}

// //bottom right RHS screen
// enigo.mouse_move_to(5120, 1440);
// thread::sleep(wait_time);
