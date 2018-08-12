use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        audio: 0,
        words: "My room was getting disordered.",
        width: 4,
        height: 5,
        exit: (2, 3),
        exit_direction: "right",
        map: vec![
            0, 0, 0, 0,
            0, 2, 0, 0,
            0, 2, 0, 0,
            9, 1, 3, 1,
            1, 1, 3, 1,
        ]
    }
}
