use specs::{Entity, WriteStorage, ReadStorage, Entities, Join};
use crate::{Position, CombatStats, StatusEffects};
use super::SkillExecutor;

pub struct MultiShot;

impl SkillExecutor for MultiShot {
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
            let targets: Vec<_> = (entities, positions)
                .join()
                .filter(|(e, pos)| {
                    *e != caster && calculate_distance(caster_pos, pos) <= 3.0
                })
                .map(|(e, _)| e)
                .take(3) // 最多攻擊3個目標
                .collect();

            for target in targets {
                if let Some(mut target_stats) = combat_stats.get_mut(target) {
                    // 多重射擊對每個目標造成遞減傷害
                    let reduced_damage = (damage as f32 * 0.8) as i32; // 傷害衰減20%
                    target_stats.hp -= reduced_damage;
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