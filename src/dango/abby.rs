use log::debug;
use rand::distr::{Distribution, Uniform};

use crate::skill::{Hook, HookCapability, Skill};

#[derive(Debug)]
pub struct Abby {
    dice: Uniform<u8>,
}

impl Abby {
    pub fn default() -> Self {
        Self {
            dice: Uniform::new_inclusive(1u8, 6).unwrap(),
        }
    }
}

impl_name!(Abby);

impl Skill for Abby {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::OnDice]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookCapability::OnDice { points } = handle.hook_cap else {
            unreachable!()
        };

        if ctx.round < 3 {
            return;
        }

        let pts = self.dice.sample(handle.rng);
        debug!("Abby points: {pts}");
        *points = pts
    }
}
