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
