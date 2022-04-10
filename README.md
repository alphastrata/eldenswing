# eldenswing

Elden Ring helper tools. [the only useful thing is the speedquit and moglvlbot]

# Prerequisites:

- Elden Ring
- Tesseract OCR ( the binary from here https://codetoprosper.com/tesseract-ocr-for-windows/)
- You've run the screenCapture.bat (https://github.com/npocmaka/batch.scripts/blob/master/hybrids/.net/c/screenCapture.bat if missing) it requires .net framework
- Rust & Cargo
- That all the paths to Tesseract and screenCapture.exe are reflecting their locations on disk for _your_ system -- amend them in the cv_utils.rs file with the text editor of your choice
- That the screen (if you have multiple) on which you play the game's top left hand corner == 0,0 in pixels, if this is not the case all the ComputerVision stuff will be out by 2560px on that axis

# USAGE:

_assuming you've cloned the repo and got all the above installed properly_

```
cd <wherever you put rustwari>
cargo build --release
./target/release/eldenswing

```

Keybindings:

```
j -- hit 3 times to speedquit
o -- hit once to speedrun Mog
i -- hit once to do a single Mog run
m -- hit once to stop EldenSwing
x -- emergency quit *this makes the app painc!() it's not a good way to quit...
```

- Whilst running it'll quit you out of the actual game into the main menu if you tripple tap 'j'.

# TODO:

- [x] Get OpenCV working ><
- [x] Get a snapshotter working (on 'l' key take a screengrab from game)
- [x] Get a basic A:B comparison going -- is SIFT usable? it's part of OpenCV
- [-] if you do get a %match, what does that mean?
- [-] Can you match a grace, then orient towards it to make it centre screen?
- [x] Ask Pavel for some ideas
- [-] Get a trimmer working (i.e crop a specific region of screen, like the compass)
- [x] Computer vision to recognise soul count?
- [x] Take footage of a manual and automated run, whip up a quick script to take frames from them
- [x] Pavel suggested taking the avg of the target from say 100 images then match on that.
- [-] get the Akaze thing working and get some numbers around how much of a match things are... what's acceptable?
- [x] Sort out Dir Structure: [Screengrabs] [Logs] [Config] [Assets] [Other]
- [x] Get monitor resolution, [-]make all magic consts relative to it
- [x] Error handling you scum
- [x] Get some logging going:

- [x] Try ps4 remote read/injection with bevy (gilrs not working on windows) -> Not working :(
- []

# Feature Ideas:

- A speed quit tool, press `j` three times consequitvely to quit faster than is humanly possible (by spamming the UI in game) therefore, _NOT_ making that FromSoftware BS warning about quitting with the task-mngr from popping up.

- Moog-run, a tool to automate Moog runs, using the Golden Wave Weapon Art.

- Gear Swap...
