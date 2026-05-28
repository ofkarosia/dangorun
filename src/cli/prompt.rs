use color_eyre::eyre::Result;
use demand::{Confirm, DemandOption, MultiSelect, Select};
use strum::VariantArray;

use crate::{cli::presets::PRESETS, dango::Dango, map::MapVariant};

fn custom_group() -> Result<Vec<Dango>> {
    let options = Dango::VARIANTS
        .iter()
        .filter_map(|d| {
            if d.is_abby() {
                None
            } else {
                Some(DemandOption::new(*d))
            }
        })
        .collect::<Vec<_>>();

    Ok(MultiSelect::new("Select dangos")
        .min(2)
        .filterable(true)
        .options(options)
        .run()?)
}

fn select_preset() -> Result<[Dango; 6]> {
    let options = PRESETS
        .iter()
        .map(|p| DemandOption::new(*p))
        .collect::<Vec<_>>();
    let preset = Select::new("Select a preset")
        .filterable(true)
        .options(options)
        .run()?;
    Ok(preset.1)
}

fn select_map() -> Result<MapVariant> {
    let options = MapVariant::VARIANTS
        .iter()
        .map(|m| DemandOption::new(*m))
        .collect::<Vec<_>>();
    let v = Select::new("Select map").options(options).run()?;
    Ok(v)
}

pub fn prompt() -> Result<(Vec<Dango>, MapVariant)> {
    // let custom = Select::new("Select dangos from").options(vec![
    //     DemandOption::new(false).label("Presets"),
    //     DemandOption::new(true).label("Custom")
    // ]).run()?;

    let use_builtin = Confirm::new("Use built-in presets?")
        .affirmative("Yes")
        .negative("No. Use custom one")
        .run()?;

    let dango_list = if use_builtin {
        select_preset()?.to_vec()
    } else {
        custom_group()?
    };

    let map = select_map()?;

    Ok((dango_list, map))
}
