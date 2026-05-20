use log::debug;

use crate::{dango::DangoName, skill::{Hook, HookCapability, HookMessage, Skill}};

#[derive(Debug, Default)]
pub struct Calcharo;

impl_name!(Calcharo);

impl Skill for Calcharo {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove]
    }
    
    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if ctx.round == 1 {
            return;
        }

        let HookMessage::BeforeMove { map } = ctx.hook_msg else { unreachable!() };
        let HookCapability::BeforeMove { step, .. } = handle.hook_cap else { unreachable!() };
        let leaderboard = map.leaderboard();

        if *leaderboard.last().unwrap() == self.name() {
            debug!("Calcharo Skill: +3");
            *step += 3
        }
    }
}
