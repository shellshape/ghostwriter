use super::Command;
use crate::util::{read_file_to_string, write_string_to_file};
use anyhow::{bail, Result};
use clap::Args;
use diffy::{apply, Patch};
use duration_string::DurationString;
use log::info;
use std::io::stdin;
use std::{fs, thread, time::Duration};

#[derive(Args, Debug)]
pub struct Replay {
    /// The file in which the recorded snapshorts
    /// // will be replayed.
    #[clap()]
    file: String,

    /// The directory of the reploay snapshort files.
    #[clap(long, short, default_value = "timeline")]
    timeline: String,

    /// Specify a delay in which the replay will be advanced.
    /// When not specified, the replay needs to be advanced
    /// manually through the console input.
    #[clap(long, short)]
    delay: Option<String>,
}

impl Command for Replay {
    fn run(&self) -> Result<()> {
        let mut files = fs::read_dir(&self.timeline)?.collect::<Result<Vec<_>, _>>()?;
        if files.is_empty() {
            bail!("The given timeline directory does not contain any snapshots.");
        }

        let mut current = String::new();
        write_string_to_file(&self.file, &current)?;

        files.sort_by_key(|a| a.file_name());

        let delay = parse_duration(&self.delay)?;
        let waiter = get_waiter(delay);

        for entry in files {
            waiter()?;
            info!(
                "Read and apply patch {} ...",
                entry.path().to_str().unwrap_or_default()
            );
            let patch = read_file_to_string(entry.path())?;
            let patch = Patch::from_str(&patch)?;
            current = apply(&current, &patch)?;
            write_string_to_file(&self.file, &current)?;
        }

        Ok(())
    }
}

fn get_waiter(d: Option<Duration>) -> Box<dyn Fn() -> Result<()>> {
    if let Some(d) = d {
        info!(
            "Advancing replay every {} ...",
            DurationString::from(d).to_string()
        );
        Box::from(move || {
            thread::sleep(d);
            Ok(())
        })
    } else {
        info!("Press [ENTER] in the console to advance replay");
        Box::from(|| {
            stdin().read_line(&mut String::new())?;
            Ok(())
        })
    }
}

fn parse_duration(d: &Option<String>) -> Result<Option<Duration>> {
    if let Some(d) = d {
        let d: Duration = DurationString::try_from(d.to_owned())
            .map_err(|e| anyhow::anyhow!(e))?
            .into();
        Ok(Some(d))
    } else {
        Ok(None)
    }
}
