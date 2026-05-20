use log::debug;

use crate::{
    dango::DangoName, map::Flag, skill::{Hook, HookCapability, HookMessage, Skill}
};

#[derive(Debug, Default)]
pub struct Luuk;

impl_name!(Luuk);

impl Skill for Luuk {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::OnDevice]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookMessage::OnDevice { flag } = ctx.hook_msg else {
            unreachable!()
        };
        let HookCapability::OnDevice { map, end } = handle.hook_cap else {
            unreachable!()
        };

        if flag == Flag::Forward {
            debug!("Luuk skill: forward, +4");
            let (flag, finish) = map.forward(self.name(), 4);
            *end = finish;

            assert!(matches!(flag, Flag::Step | Flag::Restack));

            if flag == Flag::Restack {
                debug!("Restack after Luuk skill");
                let block_index = map.position(self.name()).0;
                map.restack(block_index, handle.rng);
            }
        }

        if flag == Flag::Back {
            debug!("Luuk skill: back, -2");
            let flag = map.back(self.name(), 2);
            assert_eq!(flag, Flag::Step)
        }
    }
}
