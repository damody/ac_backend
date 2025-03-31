use specs::{Entity, WriteStorage, ReadStorage, Entities, Join};
use crate::{Position, CombatStats, StatusEffects};
use super::SkillExecutor;

pub struct Fireball;

impl SkillExecutor for Fireball {
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
            let mut nearest_target = None;
            let mut min_distance = f32::MAX;

            for (target, target_pos) in (entities, positions).join() {
                if target != caster {
                    let distance = calculate_distance(caster_pos, target_pos);
                    if distance < min_distance {
                        min_distance = distance;
                        nearest_target = Some(target);
                    }
                }
            }

            if let Some(target) = nearest_target {
                if let Some(mut target_stats) = combat_stats.get_mut(target) {
                    // 火球術造成額外的魔法傷害
                    let magic_damage = (damage as f32 * 1.5) as i32;
                    target_stats.hp -= magic_damage;
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