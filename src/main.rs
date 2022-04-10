mod controller;
mod cv_utils;
mod data_utils;
mod mohgwyn;
mod os_reader;

use anyhow::Result;
use chrono::prelude::*;
use controller::MogRun;
use data_utils::PlayerHistory;
use enigo::*;
use os_reader::read_inputs_from_os;

fn main() -> Result<()> {
    println!("Hello tarnished!");
    println!("START_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));

    // check game is running and, if it isn't relaunch it

    os_reader::check_monitors(); // TODO: not useful?

    // keyboard and event reader stuff
    let receiver = winput::message_loop::start().expect("unable to read OS events...");
    let mut enigo = Enigo::new();

    // it may look as though the data collection has unnessecary duplication, but this is to potentially allow for extensibility later on (for non Mog runs for example)
    let history: PlayerHistory = PlayerHistory::new_from(98, 87, 90, 0.0, 0.0, 0);
    let mut data = data_utils::Data::new(history);

    // construct hepler structs to make gameplay easier to control
    let player = controller::PlayerController::new();
    let gamemenu = controller::GameMenus::new();
    let mut mogrun = MogRun::new();

    let _ = os_reader::check_elden_ring_is_running(&mut enigo, &gamemenu)?;

    // This runs the actual app:
    let _ = read_inputs_from_os(
        &receiver,
        &gamemenu,
        &mut enigo,
        &player,
        &mut data,
        &mut mogrun,
        &mut history.clone(),
    );
    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    println!("--------------------------------------------------------------");
    Ok(())
}
