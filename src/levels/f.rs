use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        words: "That day she came.",
        width: 6,
        height: 4,
        exit: (4, 2),
        exit_direction: "right",
        map: vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            9, 1, 8, 1, 0, 0,
            1, 1, 1, 1, 0, 0,
        ]
    }
}
