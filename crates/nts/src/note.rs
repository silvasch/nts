use std::path::PathBuf;

use anyhow::{anyhow, Result};

pub struct Note {
    pub timestamp: jiff::Timestamp,
    pub contents: String,
}

impl Note {
    pub fn new(timestamp: jiff::Timestamp, contents: String) -> Self {
        Self {
            timestamp,
            contents,
        }
    }

    pub fn from_filepath(file_path: &PathBuf) -> Result<Self> {
        let timestamp = jiff::Timestamp::from_millisecond(
            file_path
                .file_stem()
                .ok_or(anyhow!("file does not have a file stem"))?
                .to_string_lossy()
                .parse()?,
        )?;

        let file_contents = std::fs::read_to_string(file_path)?;

        Ok(Self {
            timestamp,
            contents: file_contents,
        })
    }

    pub fn save_to_file(&self, notes_dir: &PathBuf) -> Result<()> {
        std::fs::create_dir_all(notes_dir)?;
        let file_path = notes_dir
            .join(self.timestamp.as_millisecond().to_string())
            .with_extension("txt");
        std::fs::write(file_path, &self.contents)?;
        Ok(())
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.timestamp
                .to_zoned(jiff::tz::TimeZone::system())
                .strftime("%a %b %d %H:%M:%S %Y")
        )?;

        for line in self.contents.lines() {
            writeln!(f, "> {}", line)?;
        }

        Ok(())
    }
}
