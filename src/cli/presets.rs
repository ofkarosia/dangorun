use std::fmt::Display;

use crate::dango::Dango;

macro_rules! preset {
    ($desc:literal, $($name:ident),* $(,)?) => {
        Preset($desc, [$(Dango::$name),*])
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Preset(pub &'static str, pub [Dango; 6]);

impl Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.0, self.1)
    }
}

pub static PRESETS: &[Preset] = &[
    preset!("A", Luuk, Sigrika, Denia, Hiyuki, Cartethyia, Phoebe),
    preset!("B", Chisa, Mornye, Lynae, Aemeath, Shorekeeper, Carlotta),
    preset!("C", Augusta, Iuno, Phrolova, Changli, Jinhsi, Calcharo),
    preset!(
        "CN_Farewell",
        Phoebe,
        Luuk,
        Lynae,
        Mornye,
        Phrolova,
        Changli
    ),
    preset!(
        "CN_A_Knockout",
        Augusta,
        Jinhsi,
        Hiyuki,
        Iuno,
        Calcharo,
        Cartethyia
    ),
    preset!(
        "CN_B_Knockout",
        Denia,
        Shorekeeper,
        Aemeath,
        Sigrika,
        Carlotta,
        Chisa
    ),
    preset!(
        "CN_Losers_Knockout",
        Calcharo,
        Augusta,
        Cartethyia,
        Sigrika,
        Denia,
        Chisa
    ),
    preset!(
        "CN_Losers_Final",
        Denia,
        Augusta,
        Carlotta,
        Sigrika,
        Aemeath,
        Hiyuki
    ),
    preset!(
        "CN_Losers_Farewell",
        Chisa,
        Calcharo,
        Cartethyia,
        Augusta,
        Carlotta,
        Hiyuki
    ),
    preset!(
        "CN_Final",
        Shorekeeper,
        Iuno,
        Jinhsi,
        Aemeath,
        Denia,
        Sigrika
    ),
];
