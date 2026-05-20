use log::debug;
use rand::RngExt;

use crate::skill::{Hook, HookCapability, Skill};

#[derive(Debug, Default)]
pub struct Phoebe;

impl_name!(Phoebe);

impl Skill for Phoebe {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove]
    }

    fn trigger(&mut self, _ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookCapability::BeforeMove { step, .. } = handle.hook_cap else {
            unreachable!()
        };

        if handle.rng.random() {
            debug!("Phoebe skill, +1");
            *step += 1
        }
    }
}
