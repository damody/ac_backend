mod whirlwind_slash;
mod fireball;
mod multi_shot;
mod shield_bash;

pub use whirlwind_slash::WhirlwindSlash;
pub use fireball::Fireball;
pub use multi_shot::MultiShot;
pub use shield_bash::ShieldBash;

use specs::{Entity, WriteStorage, ReadStorage, Entities};
use crate::{Position, CombatStats, StatusEffects};

pub trait SkillExecutor {
    fn execute(
        &self,
        caster: Entity,
        damage: i32,
        entities: &Entities,
        positions: &ReadStorage<Position>,
        combat_stats: &mut WriteStorage<CombatStats>,
        status_effects: &mut WriteStorage<StatusEffects>,
    );
} 