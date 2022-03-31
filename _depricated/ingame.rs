use enigo::*;
use std::thread;
use std::time::Duration;

// const REFRESH_RATE: u64 = 20; // game should be more like 16ms, this means we're slower
const COMPASS_TIK: i32 = 381;

struct Input {
    input: Enigo,
}

impl Input {
    fn new() -> Input {
        Input {
            input: Enigo::new(),
        }
    }
}

pub fn L2(input: &Input) {
    // Replicates the button from the PS4 remote you'd call for weaponart
    input.enigo.key_down(Key::Layout('p'));
    std::thread::sleep(Duration::from_secs(3));
}

// Grace/Fire helper functions
pub fn touch_grace(input: &Input) {
    /// Assumes you're within the inteaction sphere...
    println!("touch_grace");
    std::thread::sleep(Duration::from_millis(100));
    interact(&input);
    exit_grace(input);
}
pub fn interact(input: &Input) {
    /// Helper func to call the button 'e' or PS4 remote's 'x' button.
    input.enigo.key_click(Key::Layout('e'));
    println!("intearct");
}
pub fn walk_fwd(t: u64, input: &Input) {
    println!("walk_fwd for: {}", t);
    for _ in 0..(t) as usize {
        input.enigo.key_click(Key::Layout('w'));
    }
}

#[derive(Debug)]

// Camera control for movement

// Quit Game helpers
fn quit_from_main_menu(input: &Input) -> bool {
    println!("quit_from_main_menu");
    let wait_time = Duration::from_secs(4);
    input.enigo.mouse_click(MouseButton::Left);
    thread::sleep(wait_time);

    input.enigo.mouse_scroll_y(-100);
    thread::sleep(Duration::from_millis(45));

    let quit = UiButton::new(1300, 1250);
    input.enigo.mouse_move_to(quit.x, quit.y);

    true
}

pub struct Moghwyn {
    map_marker_x: i32,
    map_marker_y: i32,
}

impl Moghwyn {
    pub fn new() -> Moghwyn {
        Moghwyn {
            map_marker_x: 1240,
            map_marker_y: 820,
        }
    }
    pub fn teleport(&mut self, input: &Input) {
        //TODO: buttons into an array and loop
        input.enigo.key_click(Key::Layout('g'));
        std::thread::sleep(Duration::from_millis(90));
        input.enigo.key_click(Key::Layout('f'));
        std::thread::sleep(Duration::from_millis(90));
        input.enigo.key_click(Key::Layout('e'));
        std::thread::sleep(Duration::from_millis(90));
        input.enigo.key_click(Key::Layout('e'));
    }
    pub fn walk_to_first_stop(&mut self, input: &Input) {
        walk_fwd(200, &input);
        turn(CompassDegree::fourtyfive, LR::Left); //left?
        reset_camera(&input);
        L2(&input);
        reset_camera(&input);
        walk_fwd(400, &input);
        std::thread::sleep(Duration::from_secs(1));
        reset_camera(&input);
        L2(&input);
        std::thread::sleep(Duration::from_secs(5));
        self.teleport(&input);
    }
}
