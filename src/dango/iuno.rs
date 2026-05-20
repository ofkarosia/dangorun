use indexmap::IndexSet;
use log::debug;

use crate::{dango::{Dango, DangoName}, map::Map, skill::{Hook, HookCapability, Skill}};

#[derive(Debug, Default)]
pub struct Iuno {
    activated: bool
}

impl Iuno {
    fn refill_stack(&mut self, map: &mut Map, leaderboard: &IndexSet<Dango>, dst: usize) {
        let stack = &mut map.blocks[dst].stack;

        for id in leaderboard.iter().rev() {
            stack.push_back(*id);
        }

        map.update_pos_in(dst);
        self.activated = true;
    }
}

impl_name!(Iuno);

impl Skill for Iuno {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::FinishMove]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if self.activated {
            return;
        }

        let HookCapability::FinishMove { map, .. } = handle.hook_cap else { unreachable!() };
        
        let dst = map.position(self.name()).0;
        
        if dst < 16 {
            return;
        }
        
        let leaderboard = map.leaderboard();

        if *leaderboard.first().unwrap() == self.name() || *leaderboard.last().unwrap() == self.name() {
            return;
        }
        
        for stack in map.blocks.iter_mut().filter_map(|b| Some(&mut b.stack).filter(|s| !s.is_empty())) {
            stack.clear();
        }

        debug!("Iuno Skill: tp all");

        if ctx.round < 3 {
            self.refill_stack(map, &leaderboard, dst);
            return;
        }

        let abby_pos = map.position(Dango::Abby).0;

        if abby_pos == dst {
            map.blocks[dst].stack.push_back(Dango::Abby);
        } else {
            map.blocks[abby_pos].stack.push_back(Dango::Abby);
        }

        self.refill_stack(map, &leaderboard, dst);
    }
}
