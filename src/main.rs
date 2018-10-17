#[macro_use]
extern crate glayout;

mod game;
mod cover;
mod level;
mod levels;

extern {
    pub fn play_audio(index: i32);
}

fn run() {
    game::init();
}

pub fn main() {
    glayout::init();
    glayout::main_loop(run);
}
