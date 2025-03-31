#![allow(warnings)]
mod game_state;
mod combat;
mod turn;
mod skills;

use specs::{Component, VecStorage, World, WorldExt, Builder, System, ReadStorage, WriteStorage, Join};
use specs::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::time::{Instant, Duration};
use std::thread::sleep;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Chess {
    id: Uuid,
    name: String,
    level: u32,
    chess_type: ChessType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChessType {
    Warrior,
    Mage,
    Archer,
    Tank,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct CombatStats {
    hp: i32,
    max_hp: i32,
    attack: i32,
    defense: i32,
    magic_resist: i32,
    attack_speed: f32,
    attack_range: f32,    // 新增：攻擊距離
    mana: i32,           // 當前魔力值
    max_mana: i32,       // 最大魔力值
    skill: Skill,        // 角色專屬技能
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct StatusEffects {
    effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    effect_type: EffectType,
    duration: u32,
    magnitude: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Stun,
    Poison,
    Heal,
    AttackBuff,
    DefenseBuff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub skill_type: SkillType,
    pub damage: i32,
    pub range: f32,
    pub duration: Option<u32>,    // 如果是持續性效果，則有持續時間
    pub cooldown: u32,            // 技能冷卻時間
    pub current_cooldown: u32,    // 當前冷卻時間
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillType {
    // 戰士：旋風斬，對周圍敵人造成傷害
    WhirlwindSlash,
    // 法師：火球術，對單個目標造成大量魔法傷害
    Fireball,
    // 弓箭手：多重射擊，對多個目標造成傷害
    MultiShot,
    // 坦克：護盾衝擊，增加自身防禦並眩暈敵人
    ShieldBash,
}

// 戰鬥系統
pub struct CombatSystem;

impl<'a> System<'a> for CombatSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Chess>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, StatusEffects>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, chess, mut combat_stats, mut status_effects) = data;
        // 在這裡實現戰鬥邏輯
    }
}

fn main() {
    env_logger::init();
    
    // 創建遊戲狀態
    let mut game_state = game_state::GameState::new();
    
    // 初始化遊戲（設置4個玩家）
    game_state.initialize_game(4);
    
    // 創建分發器
    let mut dispatcher = DispatcherBuilder::new()
        .with(combat::CombatSystem, "combat_system", &[])
        .with(turn::TurnSystem, "turn_system", &["combat_system"])
        .build();

    // 初始化分發器
    dispatcher.setup(&mut game_state.world);

    // 創建一些測試棋子
    let warrior = game_state.spawn_chess(ChessType::Warrior, 0, 0);
    let mage = game_state.spawn_chess(ChessType::Mage, 1, 1);
    let archer = game_state.spawn_chess(ChessType::Archer, 2, 2);
    let tank = game_state.spawn_chess(ChessType::Tank, 3, 3);

    println!("Auto Chess Backend initialized!");
    
    // 遊戲主循環
    let mut last_time = Instant::now();
    let frame_duration = Duration::from_secs_f32(1.0 / 60.0); // 60 FPS

    loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // 更新遊戲狀態
        game_state.update(delta_time);
        
        // 運行所有系統
        dispatcher.dispatch(&mut game_state.world);
        game_state.world.maintain();

        // 限制幀率
        let elapsed = current_time.elapsed();
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }
    }
}
