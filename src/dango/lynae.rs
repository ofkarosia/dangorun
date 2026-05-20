use log::debug;
use rand::distr::{Distribution, weighted::WeightedIndex};

use crate::skill::{Hook, HookCapability, Skill};

#[derive(Debug)]
pub struct Lynae {
    dist: WeightedIndex<i32>,
}

impl Default for Lynae {
    fn default() -> Self {
        Self {
            dist: WeightedIndex::new([6, 2, 2]).unwrap(),
        }
    }
}

impl_name!(Lynae);

impl Skill for Lynae {
    fn hooks(&self) -> &'static [crate::skill::Hook] {
        &[Hook::BeforeMove]
    }

    fn trigger(&mut self, _ctx: crate::skill::Context, handle: crate::skill::Handle) {
        let HookCapability::BeforeMove { step, .. } = handle.hook_cap else {
            unreachable!()
        };

        match self.dist.sample(handle.rng) {
            0 => {
                debug!("Lynae skill, x2");
                *step *= 2
            }
            1 => {
                debug!("Lynae skill, 0");
                *step = 0
            }
            2 => {}
            _ => unreachable!(),
        }
    }
}
