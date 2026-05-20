use log::debug;
use rand::RngExt;

use crate::{dango::DangoName, map::Flag, skill::{Hook, HookCapability, HookMessage, Skill}};

#[derive(Debug, Default)]
pub struct Jinhsi;

impl_name!(Jinhsi);
impl Skill for Jinhsi {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::GlobalFinishMove]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if ctx.round == 1 {
            return;
        }

        let HookMessage::GlobalFinishMove { id } = ctx.hook_msg else { unreachable!() };
        let HookCapability::GlobalFinishMove { map, .. } = handle.hook_cap else { unreachable!() };

        let (block_index, stack_index) = map.position(self.name());
        let (target_block_index, target_stack_index) = map.position(id);
        let flag = map.blocks[block_index].flag();
        let stack = &mut map.blocks[block_index].stack;
        
        if stack.len() == 1 || *stack.back().unwrap() == self.name() || target_block_index != block_index {
            return;
        }

        let has_toppings = if flag == Flag::Restack { true } else { target_stack_index == stack_index + 1 };

        if has_toppings && handle.rng.random_bool(0.4) {
            debug!("Jinhsi skill: move to the top");
            stack.remove(stack_index);
            stack.push_back(self.name());
            map.update_pos_in(block_index);
        }
    }
}
