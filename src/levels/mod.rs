mod a;

#[derive(Clone)]
pub struct LevelData {
    pub words: &'static str,
    pub width: i32,
    pub height: i32,
    pub exit: (i32, i32),
    pub exit_direction: &'static str,
    pub map: Vec<i32>,
}

pub fn get_level_data(num: i32) -> LevelData {
    match num {
        0 => a::data(),
        _ => { panic!() }
    }
}
