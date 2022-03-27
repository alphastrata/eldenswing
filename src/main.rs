mod ingame;
mod os_reader;

use enigo::*;
use std::time::Duration;

const COMPASS_TIK: i32 = 382;

#[derive(Debug)]
enum CompassDegree {
    ninety,
    fourtyfive,
    oneeighty,
    twozeventy,
    threesixty,
}

enum LR {
    Left,
    Right,
}

fn turn(d: CompassDegree, lr: LR) {
    // The compas is broken down into 16 segments, on a 2560x1440 screen that's 382px per segment
    // NOTE: camera will be reset within this call
    let mut enigo = Enigo::new();

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

    println!("Mouse at centre");
    std::thread::sleep(Duration::from_secs(2));
    enigo.mouse_move_relative(rotation, 0);
    println!("Turning 90s{:?}", rotation);
    ingame::walk_fwd(2);
}
// +=====+======+ MAIN +=====+======+
fn main() {
    loop {
        println!("Hello, world!");
        os_reader::check_quit_call(true); // this runs its own loop....
        std::thread::sleep(Duration::from_secs(8));
    }
}
//
// fn test_square_walk() {
//     // walk in a square...
//     for i in 0..4 {
//         ingame::reset_camera();
//         // walk forward by 100
//         ingame::walk_fwd(80);
//         // turn right by 90
//         turn(CompassDegree::ninety, LR::Right);
//     }
// }

//     //standing -1 from West
//     ingame::reset_camera();
//     println!("Should be -1 from West");
//     ingame::touch_grace();
//     turn(CompassDegree::oneeighty, LR::Left);
//     println!("Should be -1 from East");

//     //at the turning point
//     ingame::walk_fwd(120);
//     turn(CompassDegree::oneeighty, LR::Left);
//     ingame::reset_camera();
//     println!("Should be -1 from West");
//     turn(CompassDegree::ninety, LR::Left);
//     println!("where are you?");
//     ingame::reset_camera();
//     println!("Should be -1 from West");
//     turn(CompassDegree::fourtyfive, LR::Left);
//     println!("where are you after prev + 45?");
//     ingame::reset_camera();
//     println!("Should be -1 from West");

//     // reverse above
//     turn(CompassDegree::ninety, LR::Right);
//     turn(CompassDegree::fourtyfive, LR::Right);
//     ingame::reset_camera();
//     println!("Should be -1 from West");
//     ingame::walk_fwd(120);
//     println!("Should be back at touching distance from grace...");
//     ingame::touch_grace();
// }

// ingame::reset_camera();
// ingame::walk_back(88);
// ingame::walk_fwd(90); // do you go slower uphill?
// ingame::touch_grace();
// ingame::quit_from_game();
