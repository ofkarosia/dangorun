use color_eyre::eyre::Result;
use indexmap::IndexMap;
use log::debug;
use logcall::logcall;
use rand::{
    Rng, SeedableRng,
    distr::{Distribution, Uniform},
    rngs::{ChaCha20Rng, ThreadRng},
    seq::IndexedRandom,
};
use std::{cell::Cell, panic};
use twox_hash::XxHash3_64;

use crate::{
    dango::{Dango, DangoMap, abby::Abby},
    logging::{self, LogMode, drain_logs, init_logger},
    map::{Flag, Map, MapVariant},
    skill::{Context, DelayedAction, Handle, Hook, Skill, SkillManager},
};

thread_local! {
    static SEED: Cell<u64> = const { Cell::new(0) }
}

pub type RngCore = ChaCha20Rng;

#[logcall]
fn teleport_abby_at_end(map: &mut Map, abby_pos: usize) {
    let id = Dango::Abby;
    let last_index = map.len() - 1;
    let stack = &mut map.blocks[abby_pos].stack;
    assert_eq!(stack.len(), 1);
    stack.pop_back();

    let stack = &mut map.blocks[last_index].stack;
    debug!("End stack: {stack:?}");
    assert!(stack.is_empty());
    stack.push_back(id);
    map.update_pos(id, last_index, 0);
}

fn round_finish_check(map: &Map, round: u8, abby_activated: bool) {
    debug!("Position map after: {:#?}", map.pos_map);
    debug!("Round {round} Leaderboard: {:?}", map.leaderboard());
    map.pos_check(abby_activated);
}

fn run(seed: u64, dango_list: Vec<Dango>, map_variant: MapVariant) -> Result<Dango> {
    debug!("Simulation starts, using seed {seed}");
    let mut rng = RngCore::seed_from_u64(seed);
    let mut dango_map: DangoMap = dango_list
        .into_iter()
        .map(|d| (d, d.create_boxed()))
        .collect();
    let mut ids = dango_map.keys().copied().collect::<Vec<_>>();
    debug!("IDs: {ids:?}");

    let mut map = map_variant.create_map(&ids)?;
    let mut skill_manager = SkillManager::new();
    skill_manager.register_skills(&dango_map);

    let dice = Uniform::new_inclusive(1u8, 3).unwrap();
    let mut points_map: IndexMap<Dango, u8>;
    let mut delayed_action = DelayedAction::default();

    let mut round = 1u8;
    let mut end = false;
    let mut abby_activated = false;

    'a: while !end {
        debug!("Round {round} starts");

        if round == 3 {
            debug!("Abby activated");
            abby_activated = true;
            let abby = Box::new(Abby::default()) as Box<dyn Skill>;
            skill_manager.register(&*abby);
            dango_map.insert(Dango::Abby, Box::new(Abby::default()));
            map.pos_map.insert(Dango::Abby, (map.len() - 1, 0));
            ids.push(Dango::Abby);
        }

        let mut ordering: Vec<Dango> = ids
            .sample(&mut rng, dango_map.len())
            .copied()
            .filter(|d| !delayed_action.next_tail_ordering.contains(d))
            .collect();

        if round == 1 {
            for id in ordering.iter().rev() {
                map.blocks[0].stack.push_back(*id);
            }
        }

        let on_dice_hooks = &skill_manager[&Hook::OnDice];
        let after_dice_hooks = &skill_manager[&Hook::AfterDice];
        let before_move_hooks = &skill_manager[&Hook::BeforeMove];
        let on_device_hooks = &skill_manager[&Hook::OnDevice];
        let finish_move_hooks = &skill_manager[&Hook::FinishMove];
        let global_finish_move_hooks = &skill_manager[&Hook::GlobalFinishMove];

        debug!("{delayed_action:?}");
        ordering.extend(delayed_action.next_tail_ordering.drain(..));

        debug!("Ordering: {ordering:?}");
        assert!(delayed_action.next_tail_ordering.is_empty());

        points_map = ids
            .iter()
            .copied()
            .map(|id| {
                if !on_dice_hooks.contains(&id) {
                    return (id, dice.sample(&mut rng));
                }

                let mut pts = 0;
                let ctx = Context::on_dice(round);
                let handle = Handle::on_dice(&mut rng, &mut pts);
                dango_map.get_mut(&id).unwrap().trigger(ctx, handle);

                (id, pts)
            })
            .collect::<IndexMap<_, _>>();

        debug!("Points map: {points_map:#?}");
        debug!("Position map: {:#?}", map.pos_map);

        for id in ordering.iter().filter(|id| after_dice_hooks.contains(id)) {
            let ctx = Context::after_dice(round, &map, &points_map);
            let handle = Handle::after_dice(&mut rng, &mut delayed_action);

            debug!("AfterDice Hook -> {id}");
            dango_map.get_mut(id).unwrap().trigger(ctx, handle);
        }

        for &id in &ordering {
            if id.is_abby() && !abby_activated {
                continue;
            }

            let initial_pts = points_map[&id];
            let mut step = initial_pts;

            if before_move_hooks.contains(&id) {
                let ctx = Context::before_move(round, &map);
                let handle = Handle::before_move(&mut rng, &mut step, &mut delayed_action);
                debug!("BeforeMove Hook -> {id}");
                dango_map.get_mut(&id).unwrap().trigger(ctx, handle);
            }

            if delayed_action.effect_set.contains(&id) && initial_pts > 1 {
                step = step.saturating_sub(1)
            }

            debug!("Moving: {id}, step {step}");

            let (flag, finish) = map.forward(id, step);
            debug!("{id} move result: Flag: {flag}, finish: {finish}");

            if finish {
                break 'a;
            }

            match flag {
                Flag::Forward | Flag::Back if on_device_hooks.contains(&id) => {
                    let ctx = Context::on_device(round, flag);
                    let handle = Handle::on_device(&mut rng, &mut map, &mut end);

                    debug!("OnDevice Hook -> {id}");
                    dango_map.get_mut(&id).unwrap().trigger(ctx, handle);
                }
                Flag::Forward if id.is_abby() => {
                    debug!("Abby back 1");
                    assert_eq!(map.back(id, 1), Flag::Step)
                }
                Flag::Forward => {
                    debug!("{id} forward 1");
                    assert_eq!(map.forward(id, 1), (Flag::Step, false))
                }
                Flag::Back if id.is_abby() => {
                    debug!("Abby forward 1");
                    assert_eq!(map.forward(id, 1), (Flag::Step, false))
                }
                Flag::Back => {
                    debug!("{id} back 1");
                    assert_eq!(map.back(id, 1), Flag::Step)
                }
                Flag::Restack => {
                    debug!("Restack triggerd");
                    let block_index = map.position(id).0;
                    map.restack(block_index, &mut rng);
                }
                Flag::Step => {}
            }

            if end {
                break 'a;
            }

            if finish_move_hooks.contains(&id) {
                let ctx = Context::finish_move(round);
                let handle = Handle::finish_move(&mut rng, &mut map, &mut delayed_action);

                debug!("FinishMove Hook -> {id}");
                dango_map.get_mut(&id).unwrap().trigger(ctx, handle);
            }

            for &id in global_finish_move_hooks {
                debug!("GlobalFinishMove Hook -> {id}");

                let ctx = Context::global_finish_move(round, id);
                let handle = Handle::global_finish_move(&mut rng, &mut map);
                dango_map.get_mut(&id).unwrap().trigger(ctx, handle);
            }
        }

        delayed_action.effect_set.clear();
        round += 1;

        if !abby_activated {
            round_finish_check(&map, round, abby_activated);
            continue;
        }

        debug!("Abby tp check");

        let last = *map.leaderboard().last().unwrap();
        let last_pos = map.position(last).0;
        let abby_pos = map.position(Dango::Abby).0;

        if abby_pos < last_pos {
            debug!("Abby tp triggered");
            teleport_abby_at_end(&mut map, abby_pos);
        }

        round_finish_check(&map, round, abby_activated);
    }

    let winner = *map.leaderboard().first().unwrap();

    Ok(winner)
}

pub fn debug_sim(
    seed: u64,
    sample: bool,
    dango_list: Vec<Dango>,
    map_variant: MapVariant,
) -> Result<()> {
    init_logger(if sample {
        LogMode::Sample
    } else {
        LogMode::Debug
    });

    panic::set_hook(Box::new(|info| {
        let logs = drain_logs();

        if logging::current_mode().unwrap().is_debug() {
            eprintln!("{logs}");
        }

        eprintln!("Log hash: {:x}", XxHash3_64::oneshot(logs.as_bytes()));
        eprintln!("{}", info);
    }));

    run(seed, dango_list, map_variant)?;
    eprintln!(
        "Log hash: {:x}",
        XxHash3_64::oneshot(drain_logs().as_bytes())
    );

    Ok(())
}

pub fn sim(rng: &mut ThreadRng, dango_list: Vec<Dango>, map_variant: MapVariant) -> Result<()> {
    init_logger(LogMode::Normal);

    let mut stats: IndexMap<Dango, usize> = IndexMap::new();

    panic::set_hook(Box::new(|info| {
        eprintln!("Simulation panicked with seed: {}", SEED.get());
        eprintln!("{}", info);
    }));

    for _ in 0..100000 {
        SEED.set(rng.next_u64());
        *stats
            .entry(run(SEED.get(), dango_list.clone(), map_variant)?)
            .or_insert(0) += 1;
    }

    stats.sort_unstable_by(|_, v1, _, v2| v2.cmp(v1));

    println!("{stats:#?}");

    Ok(())
}
