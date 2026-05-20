use color_eyre::eyre::Result;

use crate::cli::run;

mod cli;
mod dango;
mod logging;
mod map;
mod sim;
mod skill;

fn main() -> Result<()> {
    run()
}
