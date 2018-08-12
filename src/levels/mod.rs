mod a;
mod b;
mod c;
mod d;
mod e;
mod f;
mod g;
mod h;
mod i;
mod j;
mod k;
mod l;
mod m;

#[derive(Clone)]
pub struct LevelData {
    pub audio: i32,
    pub words: &'static str,
    pub width: i32,
    pub height: i32,
    pub exit: (i32, i32),
    pub exit_direction: &'static str,
    pub map: Vec<i32>,
}

pub fn get_level_data(num: i32) -> Option<LevelData> {
    match num {
        0 => Some(a::data()),
        1 => Some(b::data()),
        2 => Some(c::data()),
        3 => Some(d::data()),
        4 => Some(e::data()),
        5 => Some(f::data()),
        6 => Some(g::data()),
        7 => Some(h::data()),
        8 => Some(i::data()),
        9 => Some(j::data()),
        10 => Some(k::data()),
        11 => Some(l::data()),
        12 => Some(m::data()),
        _ => None
    }
}
