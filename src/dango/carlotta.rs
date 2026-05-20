use log::debug;
use rand::RngExt;

use crate::skill::{Hook, HookCapability, Skill};

#[derive(Debug, Default)]
pub struct Carlotta;

impl_name!(Carlotta);

impl Skill for Carlotta {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove]
    }

    fn trigger(&mut self, _ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookCapability::BeforeMove { step, .. } = handle.hook_cap else {
            unreachable!()
        };

        if handle.rng.random_bool(0.28) {
            debug!("Carlotta skill: step * 2");
            *step *= 2
        }
    }
}
