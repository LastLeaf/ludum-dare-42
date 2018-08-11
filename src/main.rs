#[macro_use]
extern crate glayout;

mod game;

lib_define_callback!(Init () {
    fn callback(&mut self, _: i32, _: i32, _: i32, _: i32) -> bool {
        game::init();
        false
    }
});

pub fn main() {
    lib!(timeout(3000, lib_callback!(Init())));
    glayout::init();
    glayout::main_loop();
}
