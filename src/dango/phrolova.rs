use log::debug;

use crate::{dango::DangoName, skill::{Hook, HookCapability, HookMessage, Skill}};

#[derive(Debug, Default)]
pub struct Phrolova;

impl_name!(Phrolova);

impl Skill for Phrolova {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if ctx.round == 1 {
            return;
        }

        let HookMessage::BeforeMove { map } = ctx.hook_msg else { unreachable!() };
        let HookCapability::BeforeMove { step, .. } = handle.hook_cap else { unreachable!() };

        let (block_index, stack_index) = map.position(self.name());
        let len = map.blocks[block_index].stack.len();

        if len > 1 && stack_index == 0 {
            debug!("Phrolova Skill: +3");
            *step += 3;
        }
    }
}
