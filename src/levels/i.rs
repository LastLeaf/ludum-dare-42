use super::LevelData;

pub fn data() -> LevelData {
    LevelData {
        audio: 1,
        words: "We filled up the room with objects and our love.",
        width: 6,
        height: 6,
        exit: (4, 4),
        exit_direction: "right",
        map: vec![
            0, 0, 3, 1, 0, 0,
            0, 2, 0, 4, 3, 1,
            4, 4, 0, 1, 2, 0,
            1, 1, 2, 3, 1, 0,
            9, 1, 8, 1, 2, 0,
            1, 1, 1, 1, 2, 2,
        ]
    }
}
