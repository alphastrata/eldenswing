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
cd <wherever you put eldenr>
cargo build --release
./target/release/eldenr.exe

```

Keybindings:

```
j -- hit 3 times to speedquit
o -- hit once to speedrun Mog ( The default number of runs to be done is 100)
i -- hit once to do a single Mog run
m -- hit once to stop EldenSwing
x -- emergency quit *this makes the app painc!() it's not a good way to quit...
F1 -- Launch game
F2 -- increase walk_one by 1
F3 -- decrease walk_one by 1
F4 -- increase walk_two by 1
F5 -- decrease walk_two by 1
F12 -- take a screengrab (used mostly for debugging / prototyping new features)
```

# TODO:

- [] finish game again to get a fucking sword
- [x] improve the game launcher -- it's shit atm..
- [] impl stuff to recognise time spent/when in menus avg loop is still 16s .. can we get it lower?
- [] UI is kinda ugly... add colours?
- [] compress screenshots data (a lot...)
- [x] keybindings to features, and for runtime adjustments

# DONE:

- [-] Can you match a grace, then orient towards it to make it centre screen?
- [-] Get a trimmer working (i.e crop a specific region of screen, like the compass)
- [-] get the Akaze thing working and get some numbers around how much of a match things are... what's acceptable?
- [-] if you do get a %match, what does that mean?
- [x] 24 hour test
- [x] Ask Pavel for some ideas
- [x] Can you speed up the loaddimes if you move the install to a faster disk...??
- [x] Computer vision to recognise soul count?
- [x] Error handling you scum
- [x] Get OpenCV working ><
- [x] Get a basic A:B comparison going -- is SIFT usable? it's part of OpenCV
- [x] Get a snapshotter working (on 'l' key take a screengrab from game)
- [x] Get monitor resolution, [-]make all magic consts relative to it
- [x] Get some logging going:
- [x] Pavel suggested taking the avg of the target from say 100 images then match on that.
- [x] Sort out Dir Structure: [Screengrabs] [Logs] [Config] [Assets] [Other]
- [x] Take footage of a manual and automated run, whip up a quick script to take frames from them
- [x] Try ps4 remote read/injection with bevy (gilrs not working on windows) -> Not working
- [x] add a check to make sure char has the correct weapon equipped
- [x] add a check to make sure souls != 0, if it does, log a death and quit out of game?
- [x] bring back the runtime commands with the key matching
- [x] merge rh into win-dev
- [x] move screenshot storage to G:/
- [x] slap togeter a decent set of graphs in python off of your .csv data...
- [x] work on the mode -- this may be the key to finding good positioning (w1, w2 and the turn...)
- []

# Feature Ideas:

- A speed quit tool, press `j` three times consequitvely to quit faster than is humanly possible (by spamming the UI in game) therefore, _NOT_ making that FromSoftware BS warning about quitting with the task-mngr from popping up.

- Moog-run, a tool to automate Moog runs, using the Golden Wave Weapon Art.

- Gear Swap...


# DISCLAIMER:
IF THIS GETS YOU BANNED, LOSES YOU SOULS, OR DOESN'T BRING YOU JOY -- I DO NOT CARE.
This is prototype lvl implementation at best, use at your own risk.
THE CONTENT OF THIS REPOSITORY IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
