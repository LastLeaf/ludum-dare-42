use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        words: "Sometimes I had to get out from the window.",
        width: 4,
        height: 5,
        exit: (2, 0),
        exit_direction: "right",
        map: vec![
            0, 0, 0, 0,
            0, 0, 2, 2,
            0, 0, 3, 1,
            9, 1, 5, 1,
            1, 1, 1, 1,
        ]
    }
}
