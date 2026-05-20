use log::debug;

use crate::{
    dango::DangoName, skill::{Hook, HookCapability, HookMessage, Skill}
};

#[derive(Debug, Default)]
pub struct Chisa {
    activated: bool
}

impl_name!(Chisa);

impl Skill for Chisa {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::AfterDice, Hook::BeforeMove]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        match ctx.hook_msg {
            HookMessage::AfterDice { points_map, .. } => {
                let pts = points_map[&self.name()];
                let min = points_map.values().min().unwrap();
        
                if pts == *min {
                    self.activated = true
                }
            },
            HookMessage::BeforeMove { .. } if self.activated => {
                let HookCapability::BeforeMove { step, .. } = handle.hook_cap else { unreachable!() };
                debug!("Chisa skill: step + 2");
                *step += 2;
                self.activated = false
            },
            HookMessage::BeforeMove { .. } => {},
            _ => unreachable!()
        }
    }
}
