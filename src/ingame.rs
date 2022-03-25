use enigo::*;
use std::thread;
use std::time::Duration;

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
// Character Controll
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

pub fn quit_from_game() -> bool {
    let mut enigo = Enigo::new();
    let refrate = 20; // game tickrate should be around 1/59.9995ths of a second
    enigo.key_click(Key::Escape);
    thread::sleep(Duration::from_millis(90)); // this menu takes a while

    //move to system
    let sys = UiButton::new(20, 1080);
    sys.move_to(refrate);
    sys.enter(refrate);

    //move to quit
    let quit = UiButton::new(4002 - 2560, 280);
    quit.move_to(refrate);
    quit.enter(refrate);

    // move to yes
    let yes = UiButton::new(1140, 720);
    yes.move_to(refrate);
    yes.enter(refrate);

    true
}

fn quit_from_main_menu() -> bool {
    let wait_time = Duration::from_secs(4);
    let mut enigo = Enigo::new();
    enigo.mouse_click(MouseButton::Left);

    thread::sleep(wait_time);

    enigo.mouse_scroll_y(-100);
    thread::sleep(Duration::from_millis(45));

    let quit = UiButton::new(1300, 1250);
    enigo.mouse_move_to(quit.x, quit.y);

    println!("quit_from_main_menu");

    true
}