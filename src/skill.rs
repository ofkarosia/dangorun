use derive_more::{Deref, DerefMut};
use indexmap::{IndexMap, IndexSet};
use std::collections::{BTreeMap, HashMap, HashSet};
use strum::VariantArray;

use crate::{
    dango::{Dango, DangoName},
    map::{Flag, Map},
    sim::RngCore,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VariantArray)]
pub enum Hook {
    OnDice,
    AfterDice,
    BeforeMove,
    OnDevice,
    FinishMove,
    GlobalFinishMove,
}

#[derive(Debug)]
pub enum HookMessage<'a, 'b> {
    OnDice,
    AfterDice {
        map: &'a Map,
        points_map: &'b IndexMap<Dango, u8>,
    },
    BeforeMove {
        map: &'a Map,
    },
    OnDevice {
        flag: Flag,
    },
    FinishMove,
    GlobalFinishMove {
        id: Dango,
    },
}

#[derive(Debug)]
pub struct Context<'a, 'b> {
    pub round: u8,
    pub hook_msg: HookMessage<'a, 'b>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn on_dice(round: u8) -> Self {
        Self {
            round,
            hook_msg: HookMessage::OnDice,
        }
    }

    pub fn after_dice(round: u8, map: &'a Map, points_map: &'b IndexMap<Dango, u8>) -> Self {
        Self {
            round,
            hook_msg: HookMessage::AfterDice { map, points_map },
        }
    }

    pub fn before_move(round: u8, map: &'a Map) -> Self {
        Self {
            round,
            hook_msg: HookMessage::BeforeMove { map },
        }
    }

    pub fn on_device(round: u8, flag: Flag) -> Self {
        Self {
            round,
            hook_msg: HookMessage::OnDevice { flag },
        }
    }

    pub fn finish_move(round: u8) -> Self {
        Self {
            round,
            hook_msg: HookMessage::FinishMove,
        }
    }

    pub fn global_finish_move(round: u8, id: Dango) -> Self {
        Self {
            round,
            hook_msg: HookMessage::GlobalFinishMove { id },
        }
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct SkillManager {
    inner: HashMap<Hook, HashSet<Dango>>,
}

impl SkillManager {
    pub fn new() -> Self {
        let mut inner = HashMap::<Hook, HashSet<Dango>>::with_capacity(Hook::VARIANTS.len());

        for hook in Hook::VARIANTS {
            inner.insert(*hook, HashSet::new());
        }

        Self { inner }
    }

    pub fn register_skills(&mut self, list: &BTreeMap<Dango, Box<dyn Skill>>) {
        for (id, dango) in list {
            for hook in dango.hooks() {
                self.inner.get_mut(hook).unwrap().insert(*id);
            }
        }
    }

    pub fn register(&mut self, dango: &dyn Skill) {
        for hook in dango.hooks() {
            self.inner.get_mut(hook).unwrap().insert(dango.name());
        }
    }
}

// NOTE: Currently it's just for Sigrika
pub type EffectSet = HashSet<Dango>;

#[derive(Debug, Default)]
pub struct DelayedAction {
    pub effect_set: EffectSet,
    pub next_tail_ordering: IndexSet<Dango>,
}

#[derive(Debug)]
pub enum HookCapability<'a, 'b> {
    OnDice {
        points: &'a mut u8,
    },
    AfterDice {
        delayed_action: &'a mut DelayedAction,
    },
    BeforeMove {
        step: &'a mut u8,
        delayed_action: &'b mut DelayedAction,
    },
    OnDevice {
        map: &'a mut Map,
        end: &'b mut bool,
    },
    FinishMove {
        map: &'a mut Map,
        delayed_action: &'b mut DelayedAction,
    },
    GlobalFinishMove {
        map: &'a mut Map,
    },
}

#[derive(Debug)]
pub struct Handle<'a, 'b, 'c> {
    pub rng: &'a mut RngCore,
    pub hook_cap: HookCapability<'b, 'c>,
}

impl<'a, 'b, 'c> Handle<'a, 'b, 'c> {
    pub fn on_dice(rng: &'a mut RngCore, points: &'b mut u8) -> Self {
        Self {
            rng,
            hook_cap: HookCapability::OnDice { points },
        }
    }

    pub fn after_dice(rng: &'a mut RngCore, delayed_action: &'b mut DelayedAction) -> Self {
        Self {
            rng,
            hook_cap: HookCapability::AfterDice { delayed_action },
        }
    }

    pub fn before_move(
        rng: &'a mut RngCore,
        step: &'b mut u8,
        delayed_action: &'c mut DelayedAction,
    ) -> Self {
        Self {
            rng,
            hook_cap: HookCapability::BeforeMove {
                step,
                delayed_action,
            },
        }
    }

    pub fn on_device(rng: &'a mut RngCore, map: &'b mut Map, end: &'c mut bool) -> Self {
        Self {
            rng,
            hook_cap: HookCapability::OnDevice { map, end },
        }
    }

    pub fn finish_move(
        rng: &'a mut RngCore,
        map: &'b mut Map,
        delayed_action: &'c mut DelayedAction,
    ) -> Self {
        Self {
            rng,
            hook_cap: HookCapability::FinishMove {
                map,
                delayed_action,
            },
        }
    }

    pub fn global_finish_move(rng: &'a mut RngCore, map: &'b mut Map) -> Self {
        Self {
            rng,
            hook_cap: HookCapability::GlobalFinishMove { map },
        }
    }
}

pub trait Skill: DangoName {
    fn hooks(&self) -> &'static [Hook];
    fn trigger(&mut self, ctx: Context, handle: Handle);
}
