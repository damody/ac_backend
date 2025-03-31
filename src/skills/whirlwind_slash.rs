use specs::{Entity, WriteStorage, ReadStorage, Entities, Join};
use crate::{Position, CombatStats, StatusEffects};
use super::SkillExecutor;

pub struct WhirlwindSlash;

impl SkillExecutor for WhirlwindSlash {
    fn execute(
        &self,
        caster: Entity,
        damage: i32,
        entities: &Entities,
        positions: &ReadStorage<Position>,
        combat_stats: &mut WriteStorage<CombatStats>,
        _status_effects: &mut WriteStorage<StatusEffects>,
    ) {
        if let Some(caster_pos) = positions.get(caster) {
            let targets: Vec<_> = (&*entities, positions)
                .join()
                .filter(|(e, pos)| {
                    *e != caster && calculate_distance(caster_pos, pos) <= 2.0
                })
                .map(|(e, _)| e)
                .collect();

            for target in targets {
                if let Some(mut target_stats) = combat_stats.get_mut(target) {
                    target_stats.hp -= damage;
                }
            }
        }
    }
}

fn calculate_distance(pos1: &Position, pos2: &Position) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    ((dx * dx + dy * dy) as f32).sqrt()
} 