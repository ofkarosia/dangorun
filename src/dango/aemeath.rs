use log::debug;

use crate::{dango::DangoName, map::Flag, skill::{Hook, HookCapability, Skill}};

#[derive(Debug, Default)]
pub struct Aemeath {
    activated: bool
}

impl_name!(Aemeath);

impl Skill for Aemeath {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::FinishMove]
    }

    fn trigger(&mut self, _ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if self.activated {
            return;
        }

        let HookCapability::FinishMove { map, .. } = handle.hook_cap else { unreachable!() };
        let (block_index, stack_index) = map.position(self.name());

        if block_index < 16 {
            return;
        }

        let leaderboard = map.leaderboard();
        let stack = &mut map.blocks[block_index].stack;
        let len = stack.len();

        let last = *stack.back().unwrap();

        if last == *leaderboard.first().unwrap() {
            return;
        };

        debug!("Aemeath skill: tp to the top of the nearest");

        if len == 1 || stack_index == len - 1 {
            stack.pop_back();
        } else if stack_index == 0 {
            stack.pop_front();
        } else {
            stack.remove(stack_index);
        }

        if !stack.is_empty() {
            map.update_pos_in(block_index);
        }

        let target = leaderboard[leaderboard.get_index_of(&last).unwrap() - 1];
        let new_block_index = map.position(target).0;
        let new_flag = map.blocks[new_block_index].flag();

        assert!(matches!(new_flag, Flag::Step | Flag::Restack));
        
        let new_stack = &mut map.blocks[new_block_index].stack;
        let new_stack_index = new_stack.len();
        new_stack.push_back(self.name());

        if new_flag == Flag::Restack {
            map.restack(new_block_index, handle.rng);
        } else {
            map.update_pos(self.name(), new_block_index, new_stack_index);
        }
        self.activated = true;
    }
}
