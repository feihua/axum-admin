use chrono::Local;
use std::fs::{OpenOptions, create_dir_all};
use std::io;
use std::path::PathBuf;
use tracing_subscriber::fmt::writer::MakeWriter;

pub struct DailyLogFile {
    directory: PathBuf,
    prefix: String,
    suffix: String,
}

impl DailyLogFile {
    pub fn new(directory: impl Into<PathBuf>, prefix: impl Into<String>, suffix: impl Into<String>) -> Self {
        Self {
            directory: directory.into(),
            prefix: prefix.into(),
            suffix: suffix.into(),
        }
    }
}

impl<'a> MakeWriter<'a> for DailyLogFile {
    type Writer = io::BufWriter<std::fs::File>;

    fn make_writer(&'a self) -> Self::Writer {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let filename = format!("{}-{}.{}", self.prefix, date, self.suffix);

        let mut path = self.directory.clone();
        if let Err(e) = create_dir_all(&path) {
            eprintln!("Failed to create log directory: {:?}", e);
        }
        path.push(filename);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap_or_else(|e| panic!("Failed to open log file {:?}: {}", path, e));

        io::BufWriter::new(file)
    }
}
