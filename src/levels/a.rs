use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        audio: 0,
        words: "Every morning I left my room for work.",
        width: 4,
        height: 5,
        exit: (2, 3),
        exit_direction: "right",
        map: vec![
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
            9, 1, 0, 0,
            1, 1, 0, 0,
        ]
    }
}
