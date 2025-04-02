use specs::{World, WorldExt, Builder, Join};
use uuid::Uuid;
use std::collections::HashMap; // Import HashMap
use crate::{Position, Chess, ChessType, CombatStats, StatusEffects, Skill, SkillType};
use crate::turn::{TurnState, TurnPhase, Player, TurnManager};
use crate::{ChannelMessage, WebsocketChannel}; // Import the ChannelMessage enum and SpecsChannel

pub struct GameState {
    pub world: World,
    pub turn_manager: TurnManager,
    pub mode: Mode,
    pub mode_timer: f32,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Selection,
    Combat,
}

impl GameState {
    pub fn new(websocket_player_channels: HashMap<String, WebsocketChannel>) -> Self {
        let mut world = World::new();
        
        // 註冊所有組件
        world.register::<Position>();
        world.register::<Chess>();
        world.register::<CombatStats>();
        world.register::<StatusEffects>();
        world.register::<TurnState>();
        world.register::<Player>();
    
        // 註冊 websocket_player_channels 作為全域變數
        world.insert(websocket_player_channels);
    
        // 創建回合管理器（設置各階段時間）
        let turn_manager = TurnManager::new(30.0, 60.0, 5.0);
        
        GameState {
            world,
            turn_manager,
            mode: Mode::Selection,
            mode_timer: 10.0, // 10 seconds for selection mode
        }
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

        use rand::seq::SliceRandom;
        use rand::thread_rng;

        // 創建所有玩家並分配隨機棋子
        let chess_types = vec![
            ChessType::Warrior,
            ChessType::Mage,
            ChessType::Archer,
            ChessType::Tank,
        ];
        for i in 0..num_players {
            self.world
                .create_entity()
                .with({
                    let (tx, rx) = tokio::sync::mpsc::channel::<ChannelMessage>(32);
                    Player {
                        id: i,
                        name_id: format!("player_{}", i), // 為每個玩家生成唯一的名稱ID
                        health: 100,
                        gold: 0,
                        level: 1,
                        experience: 0,
                    }
                })
                .build();

            // 隨機選擇一個棋子類型並生成棋子
            if let Some(random_chess_type) = chess_types.choose(&mut thread_rng()).cloned() {
                self.spawn_chess(random_chess_type, i as i32, 0); // 假設棋子初始位置為 (i, 0)
            }
        }
    }

    pub fn spawn_chess(&mut self, chess_type: ChessType, x: i32, y: i32) -> specs::Entity {
        let (combat_stats) = match chess_type {
            ChessType::Warrior => (
                CombatStats {
                    name: "Warrior".to_string(),
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
                    name: "Mage".to_string(),
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
                    name: "Archer".to_string(),
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
                    name: "Tank".to_string(),
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

        println!("Chess entity created with base stats: {:?}", combat_stats);
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
        // Update mode timer
        self.mode_timer -= delta_time;
    
    
        match self.mode {
            Mode::Selection => {
                if self.mode_timer <= 0.0 {
                    self.mode = Mode::Combat;
                    self.mode_timer = 0.0; // Reset timer
                    println!("Transitioning to Combat mode...");
                    // Start pairing players for combat
                    self.pair_players_for_combat();
                }
            }
            Mode::Combat => {
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
    }
    
    fn pair_players_for_combat(&self) {
        println!("Pairing players for combat...");
        // Implement pairing logic here
    }
} 