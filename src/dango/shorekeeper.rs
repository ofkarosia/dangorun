use rand::RngExt;

use crate::skill::{Hook, HookCapability, Skill};

#[derive(Debug, Default)]
pub struct Shorekeeper;

impl_name!(Shorekeeper);

impl Skill for Shorekeeper {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::OnDice]
    }

    fn trigger(&mut self, _ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookCapability::OnDice { points } = handle.hook_cap else {
            unreachable!()
        };

        if handle.rng.random() {
            *points = 2
        } else {
            *points = 3
        }
    }
}
