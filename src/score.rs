use serde_json::{self, json, to_string_pretty, Value};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

const CORRUPTED_DATA_ERR_MSG: &str = "The score save file is corrupted";
const UNNABLE_TO_SAVE_ERR_MSG: &str = "Unable to save the score save file";
const SCORE_SAVE_FILE_NAME: &str = "score.json";

fn get_scores() -> Value {
    let default_contents = json!({
        "player": 0,
        "computer": 0
    });
    if let Ok(mut file) = File::open(SCORE_SAVE_FILE_NAME) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(CORRUPTED_DATA_ERR_MSG);
        serde_json::from_str::<Value>(&contents).expect(CORRUPTED_DATA_ERR_MSG)
    } else {
        let mut file = File::create(SCORE_SAVE_FILE_NAME).expect(UNNABLE_TO_SAVE_ERR_MSG);
        let contents = to_string_pretty(&default_contents).unwrap();
        file.write_all(contents.as_bytes())
            .expect(CORRUPTED_DATA_ERR_MSG);
        default_contents
    }
}

pub fn get_score(subject: &str) -> u64 {
    get_scores()[subject]
        .as_u64()
        .expect(CORRUPTED_DATA_ERR_MSG)
}

pub fn increment_score(subject: &str) {
    if let Ok(mut file) = OpenOptions::new().write(true).open(SCORE_SAVE_FILE_NAME) {
        let mut score = get_scores();
        score[subject] = Value::from(score[subject].as_u64().expect(CORRUPTED_DATA_ERR_MSG) + 1);
        file.write_all(
            to_string_pretty(&score)
                .expect(CORRUPTED_DATA_ERR_MSG)
                .as_bytes(),
        )
        .expect(CORRUPTED_DATA_ERR_MSG);
    };
}
