use gilrs::{Button, Event, Gilrs};
fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    let ps4_controller = gilrs
        .gamepads()
        .find(|(_id, gamepad)| gamepad.name().contains("PS4"));

    println!("PS4 Controller is {:#?}", ps4_controller.unwrap());

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} from {}: {:?}", time, id, event);
            active_gamepad = Some(id);

            println!("\n");
        }
        // respond to gamepad input
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed PS");
                gamepad.is_pressed(Button::RightTrigger2);
                println!("RT2 pressed in response to South");
                continue;
            }
        }
    }
}
