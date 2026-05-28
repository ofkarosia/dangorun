use pastey::paste;
use std::collections::BTreeMap;
use strum::{Display, EnumIs, VariantArray};

use crate::skill::Skill;

macro_rules! create_mappings {
    ($($name:ident),* $(,)?) => {
        $(
            paste! { pub mod [<$name:lower>]; }
        )*

        #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy, Display, EnumIs, VariantArray)]
        pub enum Dango {
            $($name),*
        }

        impl Dango {
            pub fn create_boxed(self) -> Box<dyn Skill> {
                match self {
                    $(
                        Self::$name => paste! { Box::new([<$name:lower>]::$name::default()) }
                    ),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_name {
    ($s:ident) => {
        impl $crate::dango::DangoName for $s {
            fn name(&self) -> $crate::dango::Dango {
                $crate::dango::Dango::$s
            }
        }
    };
}

create_mappings!(
    Abby,
    Aemeath,
    Augusta,
    Calcharo,
    Carlotta,
    Cartethyia,
    Changli,
    Chisa,
    Denia,
    Hiyuki,
    Iuno,
    Jinhsi,
    Luuk,
    Lynae,
    Mornye,
    Phoebe,
    Phrolova,
    Shorekeeper,
    Sigrika,
);

pub trait DangoName {
    fn name(&self) -> Dango;
}

pub type DangoMap = BTreeMap<Dango, Box<dyn Skill>>;
