use log::debug;

use crate::{
    dango::{Dango, DangoName}, skill::{Hook, HookCapability, HookMessage, Skill}
};

#[derive(Debug, Default)]
pub struct Hiyuki {
    activated: bool,
}

impl_name!(Hiyuki);

impl Skill for Hiyuki {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookMessage::BeforeMove { map } = ctx.hook_msg else { unreachable!() };
        let HookCapability::BeforeMove { step, .. } = handle.hook_cap else {
            unreachable!()
        };

        if ctx.round < 3 {
            return;
        }

        if self.activated {
            debug!("Hiyuki skill, +1");
            *step += 1
        } else if map.position(self.name()).0 >= map.position(Dango::Abby).0 {
            self.activated = true;
            debug!("Hiyuki activated and +1");
            *step += 1
        }
    }
}
