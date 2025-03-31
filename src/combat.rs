use specs::{System, ReadStorage, WriteStorage, Join, Entities, Entity};
use crate::{Position, Chess, CombatStats, StatusEffects, Effect, EffectType, SkillType};
use crate::skills::{WhirlwindSlash, Fireball, MultiShot, ShieldBash, SkillExecutor};

pub struct CombatSystem;

impl<'a> System<'a> for CombatSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Chess>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, StatusEffects>,
    );

    fn run(&mut self, (entities, positions, chess, mut combat_stats, mut status_effects): Self::SystemData) {
        // 儲存所有需要處理的攻擊和技能
        let mut attacks: Vec<(Entity, Entity)> = Vec::new();
        let mut skill_casts: Vec<(Entity, SkillType, i32)> = Vec::new();

        // 第一階段：收集資訊
        {
            let combat_stats_ref = &combat_stats;
            let mut potential_actions: Vec<(Entity, bool, i32, SkillType)> = Vec::new();

            for (e1, pos1, stats1) in (&entities, &positions, combat_stats_ref).join() {
                if stats1.mana >= stats1.max_mana && stats1.skill.current_cooldown == 0 {
                    potential_actions.push((e1, true, stats1.skill.damage, stats1.skill.skill_type.clone()));
                } else {
                    for (e2, pos2, _) in (&entities, &positions, combat_stats_ref).join() {
                        if e1 != e2 && calculate_distance(pos1, pos2) <= stats1.attack_range {
                            attacks.push((e1, e2));
                            break;
                        }
                    }
                }
            }

            // 更新狀態
            for (entity, is_skill, damage, skill_type) in potential_actions {
                if let Some(mut stats) = combat_stats.get_mut(entity) {
                    if is_skill {
                        skill_casts.push((entity, skill_type, damage));
                        stats.mana = 0;
                        stats.skill.current_cooldown = stats.skill.cooldown;
                    }
                }
            }
        }

        // 第二階段：處理攻擊
        for (attacker, target) in attacks {
            if let Some(attacker_stats) = combat_stats.get(attacker) {
                let attack_damage = calculate_damage(&attacker_stats, combat_stats.get(target).unwrap());
                if let Some(mut target_stats) = combat_stats.get_mut(target) {
                    target_stats.hp -= attack_damage;
                    target_stats.mana = (target_stats.mana + 1).min(target_stats.max_mana);
                }
                if let Some(mut attacker_stats) = combat_stats.get_mut(attacker) {
                    attacker_stats.mana = (attacker_stats.mana + 5).min(attacker_stats.max_mana);
                }
            }
        }

        // 第三階段：處理技能
        for (caster, skill_type, damage) in skill_casts {
            let skill: Box<dyn SkillExecutor> = match skill_type {
                SkillType::WhirlwindSlash => Box::new(WhirlwindSlash),
                SkillType::Fireball => Box::new(Fireball),
                SkillType::MultiShot => Box::new(MultiShot),
                SkillType::ShieldBash => Box::new(ShieldBash),
            };

            skill.execute(
                caster,
                damage,
                &entities,
                &positions,
                &mut combat_stats,
                &mut status_effects,
            );
        }

        // 更新技能冷卻
        for stats in (&mut combat_stats).join() {
            if stats.skill.current_cooldown > 0 {
                stats.skill.current_cooldown -= 1;
            }
        }
    }
}

// 計算傷害
fn calculate_damage(attacker: &CombatStats, defender: &CombatStats) -> i32 {
    let base_damage = attacker.attack;
    let damage_reduction = defender.defense as f32 / 100.0;
    (base_damage as f32 * (1.0 - damage_reduction)) as i32
}

// 計算距離
fn calculate_distance(pos1: &Position, pos2: &Position) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    ((dx * dx + dy * dy) as f32).sqrt()
}

// 判斷是否在攻擊範圍內
pub fn is_in_range(attacker_pos: &Position, target_pos: &Position, range: f32) -> bool {
    calculate_distance(attacker_pos, target_pos) <= range
} 