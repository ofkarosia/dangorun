use color_eyre::eyre::Result;
use rand::Rng;

use crate::{
    cli::{args::Args, prompt::prompt},
    sim::{debug_sim, sim},
};

mod args;
mod presets;
mod prompt;

pub fn run() -> Result<()> {
    let args: Args = argh::from_env();
    let mut rng = rand::rng();

    let (dango_list, map_variant) = prompt()?;

    if args.sample {
        return debug_sim(rng.next_u64(), true, dango_list, map_variant);
    }

    if let Some(seed) = args.debug {
        return debug_sim(seed, false, dango_list, map_variant);
    }

    sim(&mut rng, dango_list, map_variant)
}
