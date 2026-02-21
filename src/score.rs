use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const APP_DIR_NAME: &str = "snake";
const SCORE_FILE_NAME: &str = "scores.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct ScoreFile {
    high_score: u32,
}

/// Returns the platform-correct score file path.
#[must_use]
pub fn scores_path() -> PathBuf {
    let mut base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    base.push(APP_DIR_NAME);
    base.push(SCORE_FILE_NAME);
    base
}

/// Loads high score from disk, returning zero on missing or malformed data.
#[must_use]
pub fn load_high_score() -> u32 {
    load_high_score_from_path(&scores_path())
}

/// Saves high score to disk, creating parent directories when needed.
pub fn save_high_score(score: u32) -> io::Result<()> {
    save_high_score_to_path(&scores_path(), score)
}

fn load_high_score_from_path(path: &Path) -> u32 {
    let Ok(raw) = fs::read_to_string(path) else {
        return 0;
    };

    match serde_json::from_str::<ScoreFile>(&raw) {
        Ok(file) => file.high_score,
        Err(_) => 0,
    }
}

fn save_high_score_to_path(path: &Path, score: u32) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let payload = ScoreFile { high_score: score };
    let json = serde_json::to_string_pretty(&payload)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    fs::write(path, json)
}
