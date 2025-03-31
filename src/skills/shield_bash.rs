use specs::{Entity, WriteStorage, ReadStorage, Entities, Join};
use crate::{Position, CombatStats, StatusEffects, Effect, EffectType};
use super::SkillExecutor;

pub struct ShieldBash;

impl SkillExecutor for ShieldBash {
    fn execute(
        &self,
        caster: Entity,
        damage: i32,
        entities: &Entities,
        positions: &ReadStorage<Position>,
        combat_stats: &mut WriteStorage<CombatStats>,
        status_effects: &mut WriteStorage<StatusEffects>,
    ) {
        if let Some(caster_pos) = positions.get(caster) {
            // 增加施法者的防禦力
            if let Some(mut caster_stats) = combat_stats.get_mut(caster) {
                caster_stats.defense += 10;
            }

            // 尋找並眩暈範圍內的敵人
            let targets: Vec<_> = (entities, positions)
                .join()
                .filter(|(e, pos)| {
                    *e != caster && calculate_distance(caster_pos, pos) <= 1.5
                })
                .map(|(e, _)| e)
                .collect();

            for target in targets {
                // 造成傷害
                if let Some(mut target_stats) = combat_stats.get_mut(target) {
                    target_stats.hp -= damage;
                }

                // 添加眩暈效果
                if let Some(mut target_effects) = status_effects.get_mut(target) {
                    target_effects.effects.push(Effect {
                        effect_type: EffectType::Stun,
                        duration: 2,
                        magnitude: 1.0,
                    });
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