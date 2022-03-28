# eldenswing

Elden Ring helper tools.

# USAGE:

_assuming you've cloned the repo and got all the above installed properly_

```
cd <wherever you put rustwari>
cargo build --release
./target/release/eldenswing

```

- Whilst running it'll quit you out of the actual game into the main menu if you tripple tap 'j'.

# TODO:

- [x] Get OpenCV working ><
- [] Get a snapshotter working (on 'l' key take a screengrab from game)
- [] Get a basic A:B comparison going -- is SIFT usable? it's part of OpenCV
- [] if you do get a %match, what does that mean?
- [] Can you match a grace, then orient towards it to make it centre screen?
- [] Ask Pavel for some ideas
- [] Get a trimmer working (i.e crop a specific region of screen, like the compass)
- [] Computer vision to recognise soul count?
- [] Sort out Dir Structure: [Screengrabs] [Logs]
- [] Get monitor resolution, make all magic consts relative to it
- [] Error handling you scum
- [] Get some logging going:

  - Startup time
  - Calls made with timestamps
  - Screengrabs taken, with locations etc.
  - Comparison calls' results imgA.png and imgB.png are ~80% match
  - Errors

- []
- []
- []
- []

# Feature Ideas:

- A speed quit tool, press `j` three times consequitvely to quit faster than is humanly possible (by spamming the UI in game) therefore, _NOT_ making that FromSoftware BS warning about quitting with the task-mngr from popping up.

- Moog-run, a tool to automate Moog runs, using the Golden Wave Weapon Art.

- Gear Swap...
