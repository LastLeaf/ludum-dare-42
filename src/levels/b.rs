use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        words: "Sometimes I needed to move some objects.",
        width: 4,
        height: 5,
        exit: (2, 3),
        exit_direction: "right",
        map: vec![
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
            9, 1, 4, 0,
            1, 1, 1, 2,
        ]
    }
}
