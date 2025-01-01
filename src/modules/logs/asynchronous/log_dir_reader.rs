use std::path::Path;

use thiserror::Error;

use crate::logs::asynchronous::{LogFileReader, LogFileReaderError};
use crate::logs::content::LogEvent;
use crate::logs::{LogDir, LogDirError, LogFile, LogFileError};

#[derive(Debug)]
pub struct LogDirReader {
    dir: LogDir,
    current_file: Option<LogFile>,
    current_reader: Option<LogFileReader>,
    is_live: bool,
    failing: bool,
}

#[derive(Debug, Error)]
pub enum LogDirReaderError {
    #[error(transparent)]
    LogFileReaderError(#[from] LogFileReaderError),

    #[error(transparent)]
    LogDirError(#[from] LogDirError),

    #[error(transparent)]
    LogFileError(#[from] LogFileError),
}

impl LogDirReader {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        LogDirReader {
            dir: LogDir::new(path.as_ref().to_path_buf()),
            current_file: None,
            current_reader: None,
            is_live: false,
            failing: false,
        }
    }

    async fn set_current_file(&mut self, journal_file: LogFile) -> Result<(), LogDirReaderError> {
        self.current_reader = Some(journal_file.create_async_reader().await?);
        self.current_file = Some(journal_file);

        Ok(())
    }

    pub fn is_reading_latest(&self) -> bool {
        self.is_live
    }

    async fn set_next_file(&mut self) -> Result<bool, LogDirReaderError> {
        let files = self.dir.journal_logs_oldest_first()?;
        let is_empty = files.is_empty();

        let length = files.len();

        for (index, file) in files.into_iter().enumerate() {
            self.is_live = length == index + 1;

            let Some(current) = &self.current_file else {
                self.set_current_file(file).await?;

                return Ok(true);
            };

            if &file > current {
                self.set_current_file(file).await?;

                return Ok(true);
            }
        }

        Ok(is_empty)
    }

    pub fn is_failing(&self) -> bool {
        self.failing
    }

    pub async fn next(&mut self) -> Option<Result<LogEvent, LogDirReaderError>> {
        loop {
            if self.current_reader.is_none() {
                match self.set_next_file().await {
                    Ok(true) => {}
                    Ok(false) => return None,
                    Err(error) => {
                        self.failing = true;
                        return Some(Err(error));
                    }
                }
            }

            let Some(reader) = &mut self.current_reader else {
                return None;
            };

            let Some(entry) = reader.next().await else {
                match self.set_next_file().await {
                    Ok(true) => continue,
                    Ok(false) => return None,
                    Err(error) => {
                        self.failing = true;
                        return Some(Err(error));
                    }
                }
            };

            return Some(entry.map_err(|e| e.into()));
        }
    }
}
