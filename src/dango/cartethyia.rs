use log::debug;
use rand::RngExt;

use crate::{
    dango::DangoName, skill::{Context, Hook, HookCapability, Skill}
};

#[derive(Debug, Default)]
pub struct Cartethyia {
    activated: bool,
}

impl_name!(Cartethyia);

impl Skill for Cartethyia {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove, Hook::FinishMove]
    }

    fn trigger(&mut self, ctx: Context, handle: crate::skill::Handle) {
        if ctx.round == 1 {
            return;
        }

        match handle.hook_cap {
            HookCapability::BeforeMove { step, .. } if self.activated => {
                if handle.rng.random_bool(0.6) {
                    debug!("Cartethyia skill, +2");
                    *step += 2
                }
            },
            HookCapability::FinishMove { map, .. } if !self.activated => {
                let leaderboard = map.leaderboard();

                if *leaderboard.last().unwrap() == self.name() {
                    debug!("Cartethyia activated");
                    self.activated = true;
                }
            },
            HookCapability::BeforeMove { .. } | HookCapability::FinishMove { .. } => {},
            _ => unreachable!()
        }
    }
}
