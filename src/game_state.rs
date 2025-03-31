use specs::{World, WorldExt, Builder, Join};
use uuid::Uuid;
use crate::{Position, Chess, ChessType, CombatStats, StatusEffects, Skill, SkillType};
use crate::turn::{TurnState, TurnPhase, Player, TurnManager};

pub struct GameState {
    pub world: World,
    pub turn_manager: TurnManager,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // 註冊所有組件
        world.register::<Position>();
        world.register::<Chess>();
        world.register::<CombatStats>();
        world.register::<StatusEffects>();
        world.register::<TurnState>();
        world.register::<Player>();

        // 創建回合管理器（設置各階段時間）
        let turn_manager = TurnManager::new(30.0, 60.0, 5.0);
        
        GameState { world, turn_manager }
    }

    pub fn initialize_game(&mut self, num_players: usize) {
        // 創建回合狀態
        self.world
            .create_entity()
            .with(TurnState {
                current_phase: TurnPhase::Preparation,
                current_player: 0,
                total_players: num_players,
                turn_number: 1,
            })
            .build();

        // 創建所有玩家
        for i in 0..num_players {
            self.world
                .create_entity()
                .with(Player {
                    id: i,
                    health: 100,
                    gold: 0,
                    level: 1,
                    experience: 0,
                })
                .build();
        }
    }

    pub fn spawn_chess(&mut self, chess_type: ChessType, x: i32, y: i32) -> specs::Entity {
        let (combat_stats) = match chess_type {
            ChessType::Warrior => (
                CombatStats {
                    hp: 100,
                    max_hp: 100,
                    attack: 15,
                    defense: 10,
                    magic_resist: 5,
                    attack_speed: 1.0,
                    attack_range: 1.0,    // 近戰攻擊距離
                    mana: 0,
                    max_mana: 100,
                    skill: Skill {
                        skill_type: SkillType::WhirlwindSlash,
                        damage: 30,
                        range: 2.0,
                        duration: None,
                        cooldown: 3,
                        current_cooldown: 0,
                    },
                }
            ),
            ChessType::Mage => (
                CombatStats {
                    hp: 70,
                    max_hp: 70,
                    attack: 8,
                    defense: 5,
                    magic_resist: 15,
                    attack_speed: 0.8,
                    attack_range: 3.0,    // 法師遠程攻擊距離
                    mana: 0,
                    max_mana: 80,
                    skill: Skill {
                        skill_type: SkillType::Fireball,
                        damage: 50,
                        range: 4.0,
                        duration: None,
                        cooldown: 4,
                        current_cooldown: 0,
                    },
                }
            ),
            ChessType::Archer => (
                CombatStats {
                    hp: 80,
                    max_hp: 80,
                    attack: 20,
                    defense: 5,
                    magic_resist: 5,
                    attack_speed: 1.2,
                    attack_range: 4.0,    // 弓箭手最遠攻擊距離
                    mana: 0,
                    max_mana: 90,
                    skill: Skill {
                        skill_type: SkillType::MultiShot,
                        damage: 25,
                        range: 3.0,
                        duration: None,
                        cooldown: 3,
                        current_cooldown: 0,
                    },
                }
            ),
            ChessType::Tank => (
                CombatStats {
                    hp: 150,
                    max_hp: 150,
                    attack: 10,
                    defense: 20,
                    magic_resist: 20,
                    attack_speed: 0.7,
                    attack_range: 1.0,    // 近戰攻擊距離
                    mana: 0,
                    max_mana: 120,
                    skill: Skill {
                        skill_type: SkillType::ShieldBash,
                        damage: 15,
                        range: 1.5,
                        duration: Some(2),
                        cooldown: 5,
                        current_cooldown: 0,
                    },
                }
            ),
        };

        self.world
            .create_entity()
            .with(Chess {
                id: Uuid::new_v4(),
                name: format!("{:?}", chess_type),
                level: 1,
                chess_type,
            })
            .with(Position { x, y })
            .with(combat_stats)
            .with(StatusEffects { effects: Vec::new() })
            .build()
    }

    pub fn remove_chess(&mut self, entity: specs::Entity) {
        if let Err(e) = self.world.delete_entity(entity) {
            log::error!("Failed to remove chess: {:?}", e);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // 獲取當前回合狀態
        let mut turn_states = self.world.write_storage::<TurnState>();
        if let Some(turn_state) = (&mut turn_states).join().next() {
            // 更新回合管理器，檢查是否需要進入下一階段
            if self.turn_manager.update(delta_time, turn_state) {
                // 當前階段結束，進入下一階段
                match turn_state.current_phase {
                    TurnPhase::Preparation => {
                        turn_state.current_phase = TurnPhase::Combat;
                    }
                    TurnPhase::Combat => {
                        turn_state.current_phase = TurnPhase::Resolution;
                    }
                    TurnPhase::Resolution => {
                        turn_state.current_phase = TurnPhase::Preparation;
                        turn_state.current_player = (turn_state.current_player + 1) % turn_state.total_players;
                        if turn_state.current_player == 0 {
                            turn_state.turn_number += 1;
                        }
                    }
                }
            }
        }
    }
} 