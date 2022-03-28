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

pub fn L2() {
    /// Replicates the button from the PS4 remote you'd call for weaponart
    let mut enigo = Enigo::new();
    let mut start = 0; // to have keys held for a certain ammount of time

    enigo.key_down(Key::Layout('p'));
    println!("keysdown");
    while start < 3 {
        println!("loop");
        // hold down for 1s
        std::thread::sleep(Duration::from_millis(334));
        start += 1;
    }
    enigo.key_up(Key::Layout('p'));
    println!("keysup");
}

// Grace/Fire helper functions
pub fn touch_grace() {
    /// Assumes you're within the inteaction sphere...
    println!("touch_grace");
    std::thread::sleep(Duration::from_millis(100));
    interact();
    exit_grace();
}
pub fn interact() {
    /// Helper func to call the button 'e' or PS4 remote's 'x' button.
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout('e'));
    println!("intearct");
}
pub fn exit_grace() {
    /// helper func to leave a grace
    std::thread::sleep(Duration::from_millis(3000)); // ensure grace menu is loaded
    println!("exit_grace");
    rh_click_menu();
    std::thread::sleep(Duration::from_millis(800));
}
pub fn rh_click_menu() {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(1280, 720); // should be centre of screen...
    std::thread::sleep(Duration::from_millis(200));
    println!("Right click fire.");
    enigo.mouse_down(MouseButton::Right);
    std::thread::sleep(Duration::from_millis(200));
    println!("Right click fired.");
    enigo.mouse_up(MouseButton::Right);
    println!("Right click release.");
    enigo.mouse_move_relative(80, -10);
    println!("Should be on B waiting 2s.");
    std::thread::sleep(Duration::from_millis(400));
    interact();
    std::thread::sleep(Duration::from_millis(200)); // interactions take time
}

// Character Controll
pub fn reset_camera() {
    let mut enigo = Enigo::new();
    println!("reset_camera");
    enigo.key_click(Key::Layout('q'));
}
pub fn walk_fwd(t: u64) {
    println!("walk_fwd for: {}", t);
    let mut enigo = Enigo::new();
    for _ in 0..(t) as usize {
        enigo.key_click(Key::Layout('w'));
    }
}
pub fn walk_back(t: u64) {
    // takes a whole number of millis, ticks at 100ms per loop
    println!("walk_back for: {}", t);
    let mut enigo = Enigo::new();
    for _ in 0..(t) as usize {
        enigo.key_click(Key::Layout('s'));
    }
}

// Camera control for movement
pub fn centre_mouse() {
    println!("centre_mouse");
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(1280, 720); // NOTE: center will be resolution dependant.. and monitor number dependant
}

// Quit Game helpers
pub fn check_quit_call() -> bool {
    let mut enigo = Enigo::new();
    if enigo.key_click(Key::Layout('j')) == () {
        //NOTE: probs a nicer way to do this?
        println!("Quit call");
        return true;
    }
    false
}
pub fn quit_from_game() -> bool {
    println!("quit_from_game");
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

    thread::sleep(Duration::from_secs(3)); // this menu takes a while
    true
}
fn quit_from_main_menu() -> bool {
    println!("quit_from_main_menu");
    let wait_time = Duration::from_secs(4);
    let mut enigo = Enigo::new();
    enigo.mouse_click(MouseButton::Left);

    thread::sleep(wait_time);

    enigo.mouse_scroll_y(-100);
    thread::sleep(Duration::from_millis(45));

    let quit = UiButton::new(1300, 1250);
    enigo.mouse_move_to(quit.x, quit.y);

    true
}
