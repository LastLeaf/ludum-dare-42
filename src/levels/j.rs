use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        words: "Now she is not here anymore.",
        width: 6,
        height: 6,
        exit: (0, 4),
        exit_direction: "left",
        map: vec![
            0, 0, 3, 1, 0, 0,
            3, 1, 4, 0, 2, 0,
            0, 2, 1, 0, 4, 4,
            0, 3, 1, 2, 1, 1,
            0, 2, 0, 0, 9, 1,
            2, 2, 0, 0, 1, 1,
        ]
    }
}
