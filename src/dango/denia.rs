use log::debug;

use crate::{
    dango::DangoName, skill::{Hook, HookCapability, HookMessage, Skill}
};

#[derive(Debug, Default)]
pub struct Denia {
    last_point: Option<u8>,
    activated: bool
}

impl_name!(Denia);

impl Skill for Denia {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::AfterDice, Hook::BeforeMove]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        match ctx.hook_msg {
            HookMessage::AfterDice { points_map, .. } => {
                let pts = points_map[&self.name()];
                
                if let Some(last_point) = self.last_point && last_point == pts {
                    self.activated = true;
                }

                self.last_point = Some(pts)
            },
            HookMessage::BeforeMove { .. } if self.activated => {
                let HookCapability::BeforeMove { step, .. } = handle.hook_cap else { unreachable!() };
                debug!("Denia skill, +2");
                *step += 2;
                self.activated = false;
            },
            HookMessage::BeforeMove { .. } => {},
            _ => unreachable!()
        }
    }
}
