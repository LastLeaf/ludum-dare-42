use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        audio: 2,
        words: "Just like the day she came.",
        width: 6,
        height: 4,
        exit: (0, 2),
        exit_direction: "left",
        map: vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 5, 1, 9, 1,
            0, 0, 1, 1, 1, 1,
        ]
    }
}
