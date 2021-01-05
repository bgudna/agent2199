# agent2199
roguelike development in rust with the libtcod library

## current status
just started - nothing running yet

### TODO
- [ ] fill this todo list
- [ ] draw a new spritesheet / character map / font
- [ ] game logo and package art / concept art

## Reminder
 * to compile just do a 'cargo run --release'
 * if setting up for the first time, might need to install SDL2 for compiling:
 * * sudo apt install libsdl2-dev
 * if running into issues with a clean Rust setup on an M1 Mac (compiling the tcod dependency)
 * * you need to change the toolchain to stable-x86_64-apple-darwin
 * * * rustup default stable-x86_64-apple-darwin