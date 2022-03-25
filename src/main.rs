use chrono::prelude::*;
use enigo::*;
use std::thread;
use std::time::Duration;
use winput::message_loop;
use winput::{Action, Vk};

struct UiButton {
    x: i32,
    y: i32,
}

impl UiButton {
    fn new(x: i32, y: i32) -> Self {
        UiButton { x, y }
    }
    fn move_to(&self, refrate: u64) {
        let mut enigo = Enigo::new();
        thread::sleep(Duration::from_millis(refrate));
        enigo.mouse_move_to(self.x, self.y);
    }
    fn enter(&self, refrate: u64) {
        let mut enigo = Enigo::new();
        thread::sleep(Duration::from_millis(refrate));
        enigo.key_click(Key::Layout('e'));
    }
}

fn quit_from_game() -> bool {
    let mut enigo = Enigo::new();
    let wait_time = Duration::from_secs(4);
    let refrate = 20; // game tickrate should be around 1/59.9995ths of a second

    thread::sleep(wait_time);
    enigo.key_click(Key::Escape);

    //move to system
    let sys = UiButton::new(20, 1080); // RHSMonitor when in 1440p
    sys.move_to(refrate);
    sys.enter(refrate);

    //move to quit
    let quit = UiButton::new(4002 - 2560, 280); // RHSMonitor when in 1440p
    quit.move_to(refrate);
    quit.enter(refrate);

    // move to yes
    let yes = UiButton::new(1140, 720); // RHSMonitor when in 1440p
    yes.move_to(refrate);
    yes.enter(refrate);

    true
}

fn move_to_farm_point(refrate: u64) -> bool {
    todo!();
}

fn read_inputs_from_os() -> bool {
    let receiver = message_loop::start().unwrap();

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
                    println!("{:?} was pressed!", vk);
                }
            }
            _ => (),
        }
    }
}

fn quit_from_main_menu() -> bool {
    let wait_time = Duration::from_secs(4);
    let mut enigo = Enigo::new();
    enigo.mouse_click(MouseButton::Left);

    thread::sleep(wait_time);

    enigo.mouse_scroll_y(-100);
    thread::sleep(Duration::from_millis(45));

    let quit = UiButton::new(1300, 1250); // RHSMonitor when in 1440p
    enigo.mouse_move_to(quit.x, quit.y);

    println!("quit_from_main_menu");

    true
}

fn reset_camera() {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('q'));
    println!("reset_camera");
}

fn walk_fwd(t: u64) {
    let mut enigo = Enigo::new();
    for _ in 0..(t) as usize {
        enigo.key_click(Key::Layout('w'));
    }
}
fn walk_back(t: u64) {
    // takes a whole number of millis, ticks at 100ms per loop
    let mut enigo = Enigo::new();
    for _ in 0..(t) as usize {
        enigo.key_click(Key::Layout('s'));
    }
}
fn main() {
    thread::sleep(Duration::from_secs(3));
    // let _ = quit_from_game();
    // let _ = quit_from_main_menu();
    println!("START: {:?}", Utc::now());

    println!("  END: {:?}", Utc::now());
}
