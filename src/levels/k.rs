use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        words: "The room is becoming empty.",
        width: 6,
        height: 6,
        exit: (0, 4),
        exit_direction: "left",
        map: vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 4, 3, 1, 0,
            0, 2, 1, 0, 3, 1,
            0, 3, 1, 0, 2, 0,
            0, 0, 5, 1, 9, 1,
            2, 0, 1, 1, 1, 1,
        ]
    }
}
