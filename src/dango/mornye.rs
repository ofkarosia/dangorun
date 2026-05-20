use crate::skill::{Hook, HookCapability, Skill};

#[derive(Debug)]
pub struct Mornye {
    dice: [u8; 3],
}

impl Default for Mornye {
    fn default() -> Self {
        Self { dice: [3, 2, 1] }
    }
}

impl_name!(Mornye);

impl Skill for Mornye {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::OnDice]
    }

    fn trigger(&mut self, _ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookCapability::OnDice { points } = handle.hook_cap else {
            unreachable!()
        };

        let pts = self.dice[0];
        *points = pts;
        self.dice.rotate_left(1);
    }
}
