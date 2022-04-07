# eldenswing

Elden Ring helper tools: Speedquit, CV-based soul counting, Mowghyn's Palace farming, and more.

# USAGE:

_assuming you've cloned the repo and got all the above installed properly_

```
cd <wherever you put eldenr>
cargo build --release
./target/release/eldenr.exe

```

Keybindings:

```
n/a
```

# TODO:

- [] bring back the runtime commands with the key matching
- [] impl stuff to recognise time spent/when in menus avg loop is still 16s .. can we get it lower?
- [] UI is kinda ugly... add colours?
- [] work on the mode -- this may be the key to finding good positioning (w1, w2 and the turn...)
- [] 24 hour test
- [] slap togeter a decent set of graphs in python off of your .csv data...
- [] move screenshot storage to G:/

# DONE:

- [x] Can you speed up the loaddimes if you move the install to a faster disk...??
- [x] add a check to make sure char has the correct weapon equipped
- [x] add a check to make sure souls != 0, if it does, log a death and quit out of game?
- [x] Get OpenCV working ><
- [x] Get a snapshotter working (on 'l' key take a screengrab from game)
- [x] Get a basic A:B comparison going -- is SIFT usable? it's part of OpenCV
- [-] if you do get a %match, what does that mean?
- [x] Can you match a grace, then orient towards it to make it centre screen?
- [x] Ask Pavel for some ideas
- [x] Get a trimmer working (i.e crop a specific region of screen, like the compass)
- [x] Computer vision to recognise soul count?
- [x] Take footage of a manual and automated run, whip up a quick script to take frames from them
- [x] Pavel suggested taking the avg of the target from say 100 images then match on that.
- [-] get the Akaze thing working and get some numbers around how much of a match things are... what's acceptable?
- [x] Sort out Dir Structure: [Screengrabs] [Logs] [Config] [Assets] [Other]
- [x] Get monitor resolution, [-]make all magic consts relative to it
- [x] Error handling you scum
- [x] Get some logging going:
- [x] Try ps4 remote read/injection with bevy (gilrs not working on windows) -> Not working :(

# Feature Ideas:

- A speed quit tool, press `j` three times consequitvely to quit faster than is humanly possible (by spamming the UI in game) therefore, _NOT_ making that FromSoftware BS warning about quitting with the task-mngr from popping up.

- Moog-run, a tool to automate Moog runs, using the Golden Wave Weapon Art.

- Gear Swap...
