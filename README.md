# eldenswing

Elden Ring helper tools.

# USAGE:

_assuming you've cloned the repo and got all the above installed properly_

NOTE: I use a colemak keyboard, and as such -- some of my keyboard bindings may differ from yours, unfortunately for you -- this app is hardcoded to mine, so--- get with the colemak program?...

```
cd <wherever you put rustwari>
cargo build --release
./target/release/eldenswing

```

Keybindings:

```
j -- hit 3 times to speedquit //Disabled
o -- hit once to speedrun Mog //Enabled
i -- hit once to walk a Mog run //Enabled
m -- hit once to stop EldenSwing //Disabled
x -- emergency quit //Disabled
```

- Whilst running it'll quit you out of the actual game into the main menu if you tripple tap 'j'.

# TODO:

- [x] Get OpenCV working ><
- [] Get a snapshotter working (on 'l' key take a screengrab from game)
- [] Get a basic A:B comparison going -- is SIFT usable? it's part of OpenCV
- [] if you do get a %match, what does that mean?
- [] Can you match a grace, then orient towards it to make it centre screen?
- [x] Ask Pavel for some ideas
- [] Get a trimmer working (i.e crop a specific region of screen, like the compass)
- [] Computer vision to recognise soul count?
- [] Take footage of a manual and automated run, whip up a quick script to take frames from them
- [] Pavel suggested taking the avg of the target from say 100 images then match on that.
- [] get the Akaze thing working and get some numbers around how much of a match things are... what's acceptable?
- [] Sort out Dir Structure: [Screengrabs] [Logs] [Config] [Assets] [Other]
- [x] Get monitor resolution, [-]make all magic consts relative to it
- [] Error handling you scum
- [] Get some logging going:
  - Startup time
  - Calls made with timestamps
  - Screengrabs taken, with locations etc.
  - Comparison calls' results imgA.png and imgB.png are ~80% match
  - Errors

- [x] Try ps4 remote read/injection with bevy (gilrs not working on windows) -> Not working :(

# Feature Ideas:

- A speed quit tool, press `j` three times consequitvely to quit faster than is humanly possible (by spamming the UI in game) therefore, _NOT_ making that FromSoftware BS warning about quitting with the task-mngr from popping up.

- Moog-run, a tool to automate Moog runs, using the Golden Wave Weapon Art.

* THE PROCESS: ~~60k/full run...

1. Moghyn port (map, triangle x, leftarrow, x)
2. Walk to ROCKONE
3. L2
4. Walk to ROCKTWO
5. L2
6. Walk to ravine bot
7. L2
   REPEAT

- Gear Swap...
