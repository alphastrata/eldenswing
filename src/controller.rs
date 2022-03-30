use chrono::prelude::*;
use enigo::*;
use std::thread;
use std::time::Duration;

const COMPASS_TIK: i32 = 381;
const REFRESH_RATE: u64 = 16;

struct UiButton {
    x: i32,
    y: i32,
}

impl UiButton {
    fn new(x: i32, y: i32) -> Self {
        UiButton { x, y }
    }
    fn move_to(&self, refrate: u64, enigo: &mut Enigo) {
        thread::sleep(Duration::from_millis(refrate));
        enigo.mouse_move_to(self.x, self.y);
    }
    fn enter(&self, refrate: u64, enigo: &mut Enigo) {
        thread::sleep(Duration::from_millis(refrate));
        enigo.key_click(Key::Layout('e'));
    }
}
pub enum CompassDegree {
    ninety,
    fourtyfive,
    oneeighty,
    twozeventy,
    threesixty,
}

pub enum LR {
    Left,
    Right,
}
pub struct PlayerController {}
impl PlayerController {
    pub fn centre_joypad(enigo: &mut Enigo) {
        enigo.mouse_move_to(1280, 720);
    }
    pub fn interact(&self, enigo: &mut Enigo, refrate: u64) {
        thread::sleep(Duration::from_millis(refrate));
        enigo.key_click(Key::Layout('e'));
    }
    pub fn new() -> PlayerController {
        PlayerController {}
    }
    // The compas is broken down into 16 segments, on a 2560x1440 screen that's 382px per segment
    // NOTE: camera will be reset within this call
    pub fn turn(&self, enigo: &mut Enigo, d: CompassDegree, lr: LR) {
        let mut rotation = match { d } {
            CompassDegree::fourtyfive => COMPASS_TIK * 2,
            CompassDegree::ninety => COMPASS_TIK * 4,
            CompassDegree::oneeighty => COMPASS_TIK * 8,
            CompassDegree::twozeventy => COMPASS_TIK * 12,
            CompassDegree::threesixty => COMPASS_TIK * 16,
        };

        let mut rotation = match { lr } {
            LR::Left => -1 * rotation,
            LR::Right => rotation,
        };
        std::thread::sleep(Duration::from_millis(REFRESH_RATE));
        enigo.mouse_move_relative(rotation, 0);
        self.walk_fwd(2, enigo);
    }
    pub fn walk_fwd(&self, t: usize, enigo: &mut Enigo) {
        for _ in 0..t as usize {
            enigo.key_click(enigo::Key::Layout('w'));
        }
    }

    pub fn L2(&self, enigo: &mut Enigo) {
        enigo.key_click(enigo::Key::Layout('p'));
    }

    pub fn reset_camera(&self, enigo: &mut Enigo) {
        enigo.key_click(enigo::Key::Layout('q'));
    }
}

pub struct GameMenus {}

impl GameMenus {
    pub fn new() -> GameMenus {
        GameMenus {}
    }
    pub fn exit_grace(&self, enigo: &mut Enigo, player: PlayerController) {
        // NOTE: not used atm...
        // to exit a grace
        std::thread::sleep(Duration::from_millis(3000)); // ensure grace menu is loaded
        println!("exit_grace");
        self.rh_click_menu(enigo, player);
        std::thread::sleep(Duration::from_millis(800));
    }
    pub fn rh_click_menu(&self, enigo: &mut Enigo, player: PlayerController) {
        // in lieu of being able to use PS4 virtualisation, we'll just use the mouse
        enigo.mouse_move_to(1280, 720); // should be centre of screen...
        std::thread::sleep(Duration::from_millis(200));
        enigo.mouse_down(MouseButton::Right);
        std::thread::sleep(Duration::from_millis(200));
        enigo.mouse_up(MouseButton::Right);

        enigo.mouse_move_relative(80, -10);
        std::thread::sleep(Duration::from_millis(400));
        player.interact(enigo, REFRESH_RATE);
        std::thread::sleep(Duration::from_millis(200)); // interactions take time
    }
    // This quits from within the game, assumes no menus are already open..
    pub fn quit_from_game(&self, enigo: &mut Enigo) {
        enigo.key_click(Key::Escape);
        thread::sleep(Duration::from_millis(90)); // this menu takes a while

        //move to system
        let sys = UiButton::new(20, 1080);
        sys.move_to(REFRESH_RATE, enigo);
        sys.enter(REFRESH_RATE, enigo);

        //move to quit
        let quit = UiButton::new(4002 - 2560, 280);
        sys.move_to(REFRESH_RATE, enigo);
        sys.enter(REFRESH_RATE, enigo);

        // move to yes
        let yes = UiButton::new(1140, 720);
        sys.move_to(REFRESH_RATE, enigo);
        sys.enter(REFRESH_RATE, enigo);

        thread::sleep(Duration::from_secs(3)); // this menu takes a while
    }
}

pub struct MogRun {
    map_marker_x: i32,
    map_marker_y: i32,
}

impl MogRun {
    pub fn new() -> MogRun {
        MogRun {
            map_marker_x: 1240,
            map_marker_y: 820,
        }
    }
    pub fn teleport(&self, enigo: &mut Enigo, player: &PlayerController) {
        //TODO: buttons into an array and loop
        enigo.key_click(Key::Layout('g'));
        std::thread::sleep(Duration::from_millis(90));
        enigo.key_click(Key::Layout('f'));
        std::thread::sleep(Duration::from_millis(90));
        enigo.key_click(Key::Layout('e'));
        std::thread::sleep(Duration::from_millis(90));
        enigo.key_click(Key::Layout('e'));
    }
    pub fn run(&self, enigo: &mut Enigo, player: &PlayerController) {
        player.walk_fwd(200, enigo);
        player.turn(enigo, CompassDegree::fourtyfive, LR::Left); //left?
        player.L2(enigo);
        player.walk_fwd(430, enigo);
        player.L2(enigo);
        std::thread::sleep(Duration::from_secs(8));
        self.teleport(enigo, player);
    }
}
