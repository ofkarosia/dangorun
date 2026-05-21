use log::debug;
use rand::RngExt;

use crate::{dango::DangoName, skill::{Hook, HookCapability, Skill}};

#[derive(Debug, Default)]
pub struct Changli {
    activated: bool
}

impl_name!(Changli);

impl Skill for Changli {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::FinishMove]
    }

    fn trigger(&mut self, _ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if self.activated {
            self.activated = false;
            return;
        }

        let HookCapability::FinishMove { map, delayed_action } = handle.hook_cap else { unreachable!() };

        let block_index = map.position(self.name()).0;
        let stack = &mut map.blocks[block_index].stack;

        if stack.len() == 1 || *stack.front().unwrap() == self.name() {
            return;
        }

        if handle.rng.random_bool(0.65) {
            debug!("Changli Skill: last ordering next round");
            delayed_action.next_tail_ordering.insert(self.name());
            self.activated = true;
        }
    }
}
