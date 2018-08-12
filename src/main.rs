#[macro_use]
extern crate glayout;

mod game;
mod cover;
mod level;
mod levels;

extern {
    pub fn play_audio(index: i32);
}

lib_define_callback!(Init () {
    fn callback(&mut self, _: i32, _: i32, _: i32, _: i32) -> bool {
        game::init();
        false
    }
});

pub fn main() {
    lib!(timeout(0, lib_callback!(Init())));
    glayout::init();
    glayout::main_loop();
}
