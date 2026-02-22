use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const APP_DIR_NAME: &str = "snake";
const SCORE_FILE_NAME: &str = "scores.json";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ScoreFile {
    high_score: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    theme_name: Option<String>,
}

/// Returns the platform-correct score file path.
#[must_use]
pub fn scores_path() -> PathBuf {
    let mut base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    base.push(APP_DIR_NAME);
    base.push(SCORE_FILE_NAME);
    base
}

/// Loads high score from disk.
///
/// Returns `Ok(0)` when the score file does not yet exist (first run).
/// Returns `Err` when the file exists but cannot be read or parsed, so the
/// caller can surface a warning before entering raw terminal mode.
pub fn load_high_score() -> io::Result<u32> {
    load_score_file_from_path(&scores_path()).map(|f| f.high_score)
}

/// Saves high score to disk, preserving any existing theme name.
pub fn save_high_score(score: u32) -> io::Result<()> {
    let path = scores_path();
    let mut file = load_score_file_from_path(&path).unwrap_or_default();
    file.high_score = score;
    write_score_file_to_path(&path, &file)
}

/// Loads the saved theme name from disk, or `None` when not set.
pub fn load_theme_name() -> io::Result<Option<String>> {
    load_score_file_from_path(&scores_path()).map(|f| f.theme_name)
}

/// Persists the selected theme name to disk, preserving the high score.
pub fn save_theme_name(name: &str) -> io::Result<()> {
    let path = scores_path();
    let mut file = load_score_file_from_path(&path).unwrap_or_default();
    file.theme_name = Some(name.to_owned());
    write_score_file_to_path(&path, &file)
}

fn load_score_file_from_path(path: &Path) -> io::Result<ScoreFile> {
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(ScoreFile::default()),
        Err(e) => return Err(e),
    };

    serde_json::from_str::<ScoreFile>(&raw)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_score_file_to_path(path: &Path, file: &ScoreFile) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(file)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    fs::write(path, json)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{ScoreFile, load_score_file_from_path, write_score_file_to_path};

    #[test]
    fn score_serialization_round_trip() {
        let path = unique_test_path("round_trip");

        let file = ScoreFile {
            high_score: 42,
            theme_name: None,
        };
        write_score_file_to_path(&path, &file).expect("score save should succeed");
        let loaded = load_score_file_from_path(&path).expect("load should succeed");

        assert_eq!(loaded.high_score, 42);
        cleanup_test_path(&path);
    }

    #[test]
    fn missing_score_file_returns_zero() {
        let path = unique_test_path("missing");
        // Deliberately do not create the file.
        let loaded =
            load_score_file_from_path(&path).expect("missing file should return Ok(default)");
        assert_eq!(loaded.high_score, 0);
    }

    #[test]
    fn malformed_score_file_returns_error() {
        let path = unique_test_path("malformed");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("test parent directory should be creatable");
        }
        fs::write(&path, "not-json").expect("test file write should succeed");

        assert!(
            load_score_file_from_path(&path).is_err(),
            "malformed file should return Err"
        );

        cleanup_test_path(&path);
    }

    #[test]
    fn theme_name_survives_round_trip() {
        let path = unique_test_path("theme_round_trip");

        let file = ScoreFile {
            high_score: 10,
            theme_name: Some("Ocean".to_owned()),
        };
        write_score_file_to_path(&path, &file).expect("save should succeed");
        let loaded = load_score_file_from_path(&path).expect("load should succeed");

        assert_eq!(loaded.theme_name.as_deref(), Some("Ocean"));
        cleanup_test_path(&path);
    }

    fn unique_test_path(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();

        std::env::temp_dir()
            .join("snake-score-tests")
            .join(format!("{label}-{nanos}.json"))
    }

    fn cleanup_test_path(path: &PathBuf) {
        let _ = fs::remove_file(path);
        if let Some(parent) = path.parent() {
            let _ = fs::remove_dir(parent);
        }
    }
}
