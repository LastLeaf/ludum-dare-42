use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        words: "My name is Coca.",
        width: 4,
        height: 5,
        exit: (3, 4),
        exit_direction: "right",
        map: vec![
            0, 0, 0, 0,
            3, 1, 0, 0,
            9, 1, 0, 0,
            1, 1, 0, 0,
            2, 0, 0, 2,
        ]
    }
}
