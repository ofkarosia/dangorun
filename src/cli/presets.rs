use std::fmt::Display;

use crate::dango::Dango;

macro_rules! create_presets {
    (
        $($desc:literal: $($name:ident),* $(,)?)*
    ) => {
        [$(
            Preset($desc, [$(Dango::$name),*])
        ),*]
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Preset(pub &'static str, pub [Dango; 6]);

impl Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.0, self.1)
    }
}

pub static PRESETS: &[Preset] = &create_presets! {
    "A": Luuk, Sigrika, Denia, Hiyuki, Cartethyia, Phoebe,
    "B": Chisa, Mornye, Lynae, Aemeath, Shorekeeper, Carlotta,
    "C": Augusta, Iuno, Phrolova, Changli, Jinhsi, Calcharo,
    "CN_Farewell": Phoebe, Luuk, Lynae, Mornye, Phrolova, Changli,
    "CN_A_Knockout": Augusta, Jinhsi, Hiyuki, Iuno, Calcharo, Cartethyia
    "CN_B_Knockout": Denia, Shorekeeper, Aemeath, Sigrika, Carlotta, Chisa,
    "CN_Losers_Knockout": Calcharo, Augusta, Cartethyia, Sigrika, Denia, Chisa
    "CN_Losers_Final": Denia, Augusta, Carlotta, Sigrika, Aemeath, Hiyuki
    "CN_Losers_Farewell": Chisa, Calcharo, Cartethyia, Augusta, Carlotta, Hiyuki
    "CN_Final": Shorekeeper, Iuno, Jinhsi, Aemeath, Denia, Sigrika
};
