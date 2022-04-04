use crate::cv_utils::GameWindow;
use crate::data_utils::PlayerHistory;
use chrono::prelude::*;
use enigo::*;
use std::thread;
use std::time::Duration;

const COMPASS_TIK: i32 = 381;
const REFRESH_RATE: u64 = 16;

// generic to hold the screen coords (in pixels) of the locations of buttons
// this must be used when doing ingame UI manipulation with the mouse
struct UiButton {
    x: i32,
    y: i32,
}

// helper methods to make buttons identified easier to interact with when using the mouse
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
// representation of % of the ingame compass that you can use to turn
pub enum CompassDegree {
    ninety,
    twentytwo,
    fourtyfive,
    oneeighty,
    twozeventy,
    threesixty,
}

// representaion of left and right
pub enum LR {
    Left,
    Right,
}
// simple struct to simplify player control
pub struct PlayerController {}
impl PlayerController {
    pub fn centre_joypad(&self, enigo: &mut Enigo) {
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
    // 1degree should be ~= to 16.993px
    // NOTE: camera will be reset within this call
    pub fn turn(&self, enigo: &mut Enigo, d: CompassDegree, lr: LR) {
        let rotation = match { d } {
            CompassDegree::fourtyfive => COMPASS_TIK * 2,
            CompassDegree::twentytwo => COMPASS_TIK * 0.5 as i32,
            CompassDegree::ninety => COMPASS_TIK * 4,
            CompassDegree::oneeighty => COMPASS_TIK * 8,
            CompassDegree::twozeventy => COMPASS_TIK * 12,
            CompassDegree::threesixty => COMPASS_TIK * 16,
        };

        let rotation = match { lr } {
            LR::Left => -1 * rotation,
            LR::Right => rotation,
        };
        std::thread::sleep(Duration::from_millis(REFRESH_RATE));
        enigo.mouse_move_relative(rotation, 0);
        self.walk_fwd(&2, enigo); // forces ingame camera to right
    }
    // Try to turn over a time-period of frames, ideally combine it with a walk so that
    // you can walk in something other than straight lines...
    pub fn turn_by_frames(
        &self,
        enigo: &mut Enigo,
        d1: CompassDegree,
        d2: CompassDegree,
        f: usize,
        lr: LR,
    ) {
        todo!();
    }
    pub fn walk_fwd(&self, t: &usize, enigo: &mut Enigo) {
        for _ in 0..t.clone() {
            enigo.key_click(enigo::Key::Layout('w'));
        }
        enigo.key_up(enigo::Key::Layout('w'));
    }

    // TODO: make a walk_fwd based on frames
    pub fn walk_by_frames(&self, f: usize, enigo: &mut Enigo) {
        for _ in 0..f as usize {
            std::thread::sleep(Duration::from_millis(REFRESH_RATE));
            enigo.key_click(enigo::Key::Layout('w'));
        }
        enigo.key_up(enigo::Key::Layout('w'));
    }
    // DEPRICATED
    pub fn run_fwd(&self, t: u64, enigo: &mut Enigo) {
        enigo.key_down(enigo::Key::Space);
        enigo.key_down(enigo::Key::Layout('w'));
        println!("running");
        std::thread::sleep(Duration::from_millis(REFRESH_RATE * 2));

        std::thread::sleep(Duration::from_millis(t));

        std::thread::sleep(Duration::from_millis(REFRESH_RATE * 2));
        enigo.key_up(enigo::Key::Space);
        enigo.key_up(enigo::Key::Layout('w'));
        println!("Keys up..");
    }
    pub fn l2(&self, enigo: &mut Enigo) {
        enigo.key_click(enigo::Key::Layout('p'));
    }

    pub fn reset_camera(&self, enigo: &mut Enigo) {
        enigo.key_click(enigo::Key::Layout('q'));
    }
}

pub struct GameMenus {}
// methods for interacting with gamemenus
impl GameMenus {
    pub fn new() -> GameMenus {
        GameMenus {}
    }
    pub fn exit_grace(&self, enigo: &mut Enigo, player: PlayerController) {
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

        // move to system
        let sys = UiButton::new(20, 1080);
        sys.move_to(REFRESH_RATE, enigo);
        sys.enter(REFRESH_RATE, enigo);

        // move to quit
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

// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone, Copy)]
pub struct MogRun {
    pub current_run_end_utc: DateTime<Utc>,
    pub current_run_number: usize,
    pub current_run_start_utc: DateTime<Utc>,
    // pub est_endtime: DateTime<Utc>, // num_runs * avg_time_per_run - runs_done //calculate this
    pub est_goldeye_spawns: usize,
    pub est_time_remaining: Duration,
    pub run_count_total_thusfar: usize,
    pub run_count_total_absolute: usize, // num of runs controlling the range of the loop
    // pub runs_per_minute: f64, // calculate this
    pub souls_avg_per_run: usize,
    pub souls_best_thusfar: usize,
    pub souls_last_run: i64,
    pub souls_this_run: i64,
    // pub souls_total_all_runs: Vec<i64>,
    // pub souls_vs_last_run: usize, calculate this...
    pub souls_worst_thusfar: usize,
    pub starting_souls: usize, // they may start a run with some souls on the counter
    pub time_app_spartup_utc: DateTime<Utc>,
    // pub time_avg_per_run: Duration, // calculate this
    pub time_best_thusfar: Duration,
    pub time_worst_thusfar: Duration,
    pub turn_angle: f64,
    pub walk_one: f64,
    pub walk_two: f64,
    pub yield_total: usize,
}

// helpers to facilitate a Moghywn run
impl MogRun {
    pub fn new() -> MogRun {
        MogRun {
            current_run_end_utc: Utc::now(),
            current_run_number: 1,
            current_run_start_utc: Utc::now(),
            // est_endtime: Utc::now(),
            est_goldeye_spawns: 1,
            est_time_remaining: Duration::from_secs(0),
            run_count_total_thusfar: 1,
            run_count_total_absolute: 1,
            // runs_per_minute: 0.0,
            souls_avg_per_run: 1,
            souls_best_thusfar: 1,
            souls_last_run: 1,
            souls_this_run: 1,
            // souls_total_all_runs: vec![],
            // souls_vs_last_run: 0,
            souls_worst_thusfar: 1,
            starting_souls: 1,
            time_app_spartup_utc: Utc::now(),
            // time_avg_per_run: Duration::from_secs(0),
            time_best_thusfar: Duration::from_secs(0),
            time_worst_thusfar: Duration::from_secs(0),
            turn_angle: 0.0,
            walk_one: 0.0,
            walk_two: 0.0,
            yield_total: 0,
        }
    }
    // Teleport to Moghywn's Palace to set up, always called at the end or run() and speedrun() to reset the area,
    // and the player location
    pub fn teleport(&self, enigo: &mut Enigo, player: &PlayerController) {
        //TODO: buttons into an array and loop
        // player.reset_camera(enigo);
        std::thread::sleep(Duration::from_millis(40));
        enigo.key_click(Key::Layout('g'));
        std::thread::sleep(Duration::from_millis(40));
        enigo.key_click(Key::Layout('f'));
        std::thread::sleep(Duration::from_millis(40));
        enigo.key_click(Key::Layout('e'));
        std::thread::sleep(Duration::from_millis(40));
        enigo.key_click(Key::Layout('e'));
    }
    // Perform a Moghywn run
    pub fn run(&self, enigo: &mut Enigo, player: &PlayerController, history: PlayerHistory) {
        player.walk_fwd(&history.walk1, enigo);
        player.turn(enigo, CompassDegree::fourtyfive, LR::Left); //left?
                                                                 // player.l2(enigo);
        player.walk_fwd(&history.walk2, enigo);
        // screengrab logic must go here...

        let _ = GameWindow::screengrab("starting_souls".into(), "png".into(), "".into())
            .expect("unable to screengrab");
        std::thread::sleep(Duration::from_millis(REFRESH_RATE * 2));
        player.l2(enigo);
        std::thread::sleep(Duration::from_millis(7400));
        self.teleport(enigo, player);
    }
}
