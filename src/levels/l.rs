use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        audio: 2,
        words: "But my heart is still filled up with her love.",
        width: 6,
        height: 6,
        exit: (0, 4),
        exit_direction: "left",
        map: vec![
            0, 0, 0, 0, 5, 1,
            0, 0, 0, 0, 1, 1,
            0, 4, 5, 1, 5, 1,
            0, 1, 1, 1, 1, 1,
            5, 1, 5, 1, 9, 1,
            1, 1, 1, 1, 1, 1,
        ]
    }
}
