#[macro_use]
extern crate glayout;

mod game;
mod cover;
mod level;
mod levels;

pub fn play_audio(_index: i32) {
    // TODO
}

fn run() {
    game::init();
}

pub fn main() {
    glayout::init();
    glayout::main_loop(run);
}
