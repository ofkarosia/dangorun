use crate::{
    dango::DangoName, skill::{Hook, HookCapability, HookMessage, Skill}
};

#[derive(Debug, Default)]
pub struct Sigrika;

impl_name!(Sigrika);

impl Skill for Sigrika {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::AfterDice]
    }

    fn trigger(&mut self, ctx: crate::skill::Context, handle: crate::skill::Handle) {
        if ctx.round == 1 {
            return;
        }

        let HookMessage::AfterDice { map, .. } = ctx.hook_msg else { unreachable!() };
        let HookCapability::AfterDice { delayed_action } = handle.hook_cap else {
            unreachable!()
        };

        let set = &mut delayed_action.effect_set;
        let leaderboard = map.leaderboard();
        let ranking = leaderboard.get_index_of(&self.name()).unwrap();

        match ranking {
            1 => {
                set.insert(leaderboard[0]);
            }
            2.. => {
                set.insert(leaderboard[ranking - 1]);
                set.insert(leaderboard[ranking - 2]);
            }
            _ => {}
        };
    }
}
