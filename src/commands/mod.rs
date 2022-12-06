pub mod record;
pub mod replay;

use anyhow::Result;

pub trait Command {
    fn run(&self) -> Result<()>;
}
