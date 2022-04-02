//
// loop {
//     // limit the loop rate to 60fps
//     std::thread::sleep(Duration::from_millis(REFRESH_RATE / 22));

//     // Match on keyboard events
//     match os_reader::read_inputs_from_os(&receiver, true) {
//         Vk::J => {
//             //Ingame quit
//             if q_count >= 3 {
//                 println!("QUIT_CALL{:^40?}.", Utc::now().format("%H:%M:%S"));
//                 gamemenu.quit_from_game(&mut enigo);
//             } else {
//                 q_count += 1;
//                 println!("Q_COUNT: {:?}", q_count);
//             }
//         }
//         Vk::O => mogrun.speedrun(&mut enigo, &player, RUNS),
//         Vk::I => mogrun.run(&mut enigo, &player, 190, 420),
//         Vk::M => mogrun.teleport(&mut enigo, &player),
//         //Emergency panic on k
//         Vk::K => panic!(),

//         //Screengrab
//         Vk::L => os_reader::take_screenshot(&one_frame.clone()).unwrap(),
//         _ => (),
//     }
// }
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

fn ps4_test_gamepad() {
    let mut engine = GamepadEngine::new();

    loop {
        engine.update().unwrap();

        for gamepad in engine.gamepads() {
            for (key, button) in gamepad.buttons() {
                if button.is_just_pressed() {
                    println!("Just Pressed: {:?}", key);
                }
            }

            for (_, joystick) in gamepad.joysticks() {
                println!("{:?}", joystick);
            }
        }

        std::thread::sleep(Duration::from_millis(16)); // interesting that this was hardcoded in from the example...
    }
}
fn gilrs_test() {
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
        // Examine new events
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed (XBox - A, PS - X)");
            }
        }
    }
}
