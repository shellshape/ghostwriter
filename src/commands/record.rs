use super::Command;
use crate::util::{read_file_to_string, write_string_to_file};
use anyhow::Result;
use clap::Args;
use diffy::create_patch;
use log::info;
use notify::{
    event::{AccessKind, AccessMode},
    Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Args, Debug)]
pub struct Record {
    /// The file to be watched for changes.
    #[clap()]
    file: String,

    /// The directory where the snapshots will be stored.
    #[clap(long, short, default_value = "timeline")]
    timeline: String,
}

impl Command for Record {
    fn run(&self) -> Result<()> {
        let timeline_path = Path::new(&self.timeline);
        if !timeline_path.exists() {
            fs::create_dir_all(timeline_path)?;
        }

        let file_path = Path::new(&self.file);

        let mut previous = self.create_diff(file_path, timeline_path, "")?;

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(file_path, RecursiveMode::NonRecursive)?;

        info!("Watching file {} for changes ...", &self.file);

        for res in rx {
            if let Err(err) = res {
                return Err(err.into());
            }

            let event = res.unwrap();
            if let EventKind::Access(AccessKind::Close(AccessMode::Write)) = event.kind {
                previous = self.create_diff(file_path, timeline_path, &previous)?;
            }
        }

        Ok(())
    }
}

impl Record {
    fn create_diff(
        &self,
        file_path: &Path,
        timeline_path: &Path,
        previous: &str,
    ) -> Result<String> {
        let current = read_file_to_string(file_path)?;

        let patch = create_patch(previous, &current);
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let timeline_path = timeline_path.join(now.as_millis().to_string());
        info!(
            "Write chages to {} ...",
            timeline_path.to_str().unwrap_or_default()
        );
        write_string_to_file(timeline_path, patch)?;

        Ok(current)
    }
}
