use log::debug;

use crate::{dango::DangoName, skill::{Hook, HookCapability, HookMessage, Skill}};

#[derive(Debug, Default)]
pub struct Augusta {
    activated: bool
}

impl_name!(Augusta);

impl Skill for Augusta {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove]
    }
    
    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if ctx.round == 1 {
            return;
        }

        if self.activated {
            self.activated = false;
            return;
        }

        let HookMessage::BeforeMove { map } = ctx.hook_msg else { unreachable!() };
        let HookCapability::BeforeMove { step, delayed_action } = handle.hook_cap else { unreachable!() };

        let (block_index, stack_index) = map.position(self.name());
        let len = map.blocks[block_index].stack.len();

        if len > 1 && stack_index == len - 1 {
            debug!("Augusta Skill: 0 step, last ordering next round");
            *step = 0;
            delayed_action.next_tail_ordering.insert(self.name());
            self.activated = true;
        }
    }
}
