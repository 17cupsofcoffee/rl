# rl
This is a simple (and extremely work-in-progress) roguelike, written in Rust.

It uses:
* [GGEZ](https://github.com/ggez/ggez/) - rendering/input/windowing/etc
* [Specs](https://github.com/slide-rs/specs) - ECS

## Building
To build this project, you'll need the SDL2 development libraries set up on your machine. A good set of instructions can be found [in the SDL2 crate's readme](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).

Once you have those, you should be able to `cargo run` the project.