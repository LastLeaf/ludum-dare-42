use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        audio: 1,
        words: "Our room seemed a little larger.",
        width: 6,
        height: 6,
        exit: (4, 4),
        exit_direction: "right",
        map: vec![
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0,
            9, 1, 8, 1, 4, 4,
            1, 1, 1, 1, 1, 1,
        ]
    }
}
