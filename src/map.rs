use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use color_eyre::eyre::Result;
use indexmap::IndexSet;
use log::debug;
use rand::{rngs::ChaCha20Rng, seq::SliceRandom};
use strum::{Display, EnumString, VariantArray};

use crate::dango::Dango;

#[derive(Debug, EnumString, Clone, Copy, PartialEq, Eq, Display)]
pub enum Flag {
    Step,
    Forward,
    Back,
    Restack,
}

#[derive(Debug, Clone)]
pub struct Block {
    flag: Flag,
    pub stack: VecDeque<Dango>,
}

impl Block {
    pub fn flag(&self) -> Flag {
        self.flag
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub blocks: Vec<Block>,
    len: usize,
    pub pos_map: HashMap<Dango, (usize, usize)>,
}

impl Map {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn update_pos(&mut self, id: Dango, block_index: usize, stack_index: usize) {
        debug!("Update pos for {id}: ({block_index}, {stack_index})");
        *self.pos_map.get_mut(&id).unwrap() = (block_index, stack_index)
    }

    pub fn update_pos_in(&mut self, block_index: usize) {
        for (stack_index, id) in self.blocks[block_index].stack.iter().enumerate() {
            debug!("Update pos for {id}: ({block_index}, {stack_index})");
            *self.pos_map.get_mut(id).unwrap() = (block_index, stack_index)
        }
    }

    pub fn pos_check(&self, abby_activated: bool) {
        debug!("{:?}", self.blocks);
        for (id, (block_index, stack_index)) in self
            .pos_map
            .iter()
            .filter(|(d, _)| !abby_activated && **d == Dango::Abby)
        {
            debug!("Checking pos of {id}");
            assert_eq!(*id, self.blocks[*block_index].stack[*stack_index])
        }
    }

    fn abby_move(&mut self, step: u8, forward: bool) -> Flag {
        let id = Dango::Abby;
        let block_index = self.position(id).0;

        if block_index == 0 {
            return Flag::Step;
        }

        let final_block_index = if forward {
            block_index.saturating_sub(step as usize)
        } else {
            let r = block_index + step as usize;
            assert!(r < self.len - 1);
            r
        };

        let mut last_block_index = block_index;

        let step = if forward {
            step.min((block_index - final_block_index) as u8)
        } else {
            step
        };

        for i in 1..=step {
            let new_block_index = if forward {
                block_index - i as usize
            } else {
                block_index + i as usize
            };

            if self.blocks[new_block_index].stack.is_empty() {
                continue;
            }

            let [new_block, block] = self
                .blocks
                .get_disjoint_mut([new_block_index, last_block_index])
                .unwrap();

            if block.stack.len() > 1 {
                new_block.stack.extend(block.stack.drain(1..));
            }

            block.stack.pop_back();
            new_block.stack.push_front(id);
            last_block_index = new_block_index;
        }

        if final_block_index != last_block_index {
            let [final_block, last_block] = self
                .blocks
                .get_disjoint_mut([final_block_index, last_block_index])
                .unwrap();

            final_block.stack.append(&mut last_block.stack);
        }

        self.update_pos_in(final_block_index);

        self.blocks[final_block_index].flag
    }

    fn abby_forward(&mut self, step: u8) -> Flag {
        assert_ne!(step, 0);
        self.abby_move(step, true)
    }

    fn abby_back(&mut self, step: u8) -> Flag {
        assert_eq!(step, 1);
        self.abby_move(step, false)
    }

    pub fn restack(&mut self, block_index: usize, rng: &mut ChaCha20Rng) {
        let stack = &mut self.blocks[block_index].stack;

        let shuffle_start = if *stack.front().unwrap() == Dango::Abby {
            1usize
        } else {
            0
        };

        let stack = &mut stack.make_contiguous()[shuffle_start..];

        if stack.len() == 1 {
            debug!("Restack skipped");
            return;
        }

        stack.shuffle(rng);
        self.update_pos_in(block_index);
    }

    pub fn forward(&mut self, id: Dango, step: u8) -> (Flag, bool) {
        if id == Dango::Abby {
            return (self.abby_forward(step), false);
        }

        let (block_index, stack_index) = self.position(id);

        if step == 0 {
            return (self.blocks[block_index].flag, false);
        }

        let count = self.blocks[block_index].stack.len();
        let last_block_index = self.len - 1;

        if stack_index == count - 1 {
            debug!("{id} is top: ({block_index}, {stack_index})");
            self.blocks[block_index].stack.pop_back();
            let new_block_index = block_index + step as usize;

            if new_block_index >= last_block_index {
                self.blocks.last_mut().unwrap().stack.push_back(id);
                return (Flag::Step, true);
            }

            let new_flag = self.blocks[new_block_index].flag;
            let new_stack = &mut self.blocks[new_block_index].stack;
            let new_stack_index = new_stack.len();
            new_stack.push_back(id);
            self.update_pos(id, new_block_index, new_stack_index);
            return (new_flag, false);
        }

        let new_block_index = block_index + step as usize;

        if new_block_index >= last_block_index {
            let [block, final_block] = self
                .blocks
                .get_disjoint_mut([block_index, last_block_index])
                .unwrap();
            final_block.stack.extend(block.stack.drain(stack_index..));
            return (Flag::Step, true);
        }

        let [block, new_block] = self
            .blocks
            .get_disjoint_mut([block_index, new_block_index])
            .unwrap();

        let new_flag = new_block.flag;
        new_block.stack.extend(block.stack.drain(stack_index..));
        self.update_pos_in(new_block_index);

        (new_flag, false)
    }

    pub fn back(&mut self, id: Dango, step: u8) -> Flag {
        if id == Dango::Abby {
            return self.abby_back(step);
        }

        let (block_index, stack_index) = self.position(id);
        let count = self.blocks[block_index].stack.len();

        let new_block_index = block_index.checked_sub(step as usize).unwrap();
        assert_ne!(new_block_index, 0);

        if stack_index == count - 1 {
            debug!("{id} is top: ({block_index}, {stack_index})");
            self.blocks[block_index].stack.pop_back();

            let new_flag = self.blocks[new_block_index].flag;
            let new_stack = &mut self.blocks[new_block_index].stack;
            let new_stack_index = new_stack.len();
            new_stack.push_back(id);
            self.update_pos(id, new_block_index, new_stack_index);
            return new_flag;
        }

        let [block, new_block] = self
            .blocks
            .get_disjoint_mut([block_index, new_block_index])
            .unwrap();

        let new_flag = new_block.flag;
        new_block.stack.extend(block.stack.drain(stack_index..));
        self.update_pos_in(new_block_index);

        new_flag
    }

    pub fn position(&self, id: Dango) -> (usize, usize) {
        *self.pos_map.get(&id).unwrap()
    }

    pub fn leaderboard(&self) -> IndexSet<Dango> {
        let mut set = IndexSet::with_capacity(self.pos_map.len());

        for block in self.blocks.iter().rev().filter(|b| !b.stack.is_empty()) {
            for id in block.stack.iter().rev().filter(|id| **id != Dango::Abby) {
                set.insert(*id);
            }
        }

        set
    }
}

pub fn init_map_with(ids: &[Dango], map_file: &str) -> Result<Map> {
    let line_count = map_file.lines().count();
    let mut blocks = Vec::with_capacity(line_count);

    for l in map_file.lines() {
        blocks.push(Block {
            flag: Flag::from_str(l)?,
            stack: VecDeque::new(),
        })
    }

    let len = blocks.len();
    let pos_map = ids.iter().map(|i| (*i, (0, 0))).collect::<HashMap<_, _>>();

    Ok(Map {
        blocks,
        len,
        pos_map,
    })
}

pub fn init_group_map(ids: &[Dango]) -> Result<Map> {
    init_map_with(ids, include_str!("../maps/group.txt"))
}

pub fn init_knockout_map(ids: &[Dango]) -> Result<Map> {
    init_map_with(ids, include_str!("../maps/knockout.txt"))
}

#[derive(Debug, Clone, Copy, Display, VariantArray)]
pub enum MapVariant {
    Group,
    Knockout,
}

impl MapVariant {
    pub fn create_map(self, ids: &[Dango]) -> Result<Map> {
        match self {
            Self::Group => init_group_map(ids),
            Self::Knockout => init_knockout_map(ids),
        }
    }
}
