use specs::{Component, System, VecStorage, WriteStorage, ReadStorage, Join, Entity};
use serde::{Serialize, Deserialize};

// 回合狀態組件
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct TurnState {
    pub current_phase: TurnPhase,
    pub current_player: usize,
    pub total_players: usize,
    pub turn_number: u32,
}

// 回合階段
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TurnPhase {
    Preparation,    // 準備階段：玩家可以購買、升級、放置棋子
    Combat,         // 戰鬥階段：棋子自動戰鬥
    Resolution,     // 結算階段：結算傷害、獎勵等
}

// 玩家組件
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Player {
    pub id: usize,
    pub health: i32,
    pub gold: i32,
    pub level: u32,
    pub experience: u32,
}

// 回合系統
pub struct TurnSystem;

impl<'a> System<'a> for TurnSystem {
    type SystemData = (
        WriteStorage<'a, TurnState>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut turn_states, players): Self::SystemData) {
        // 只應該有一個回合狀態實體
        if let Some(turn_state) = (&mut turn_states).join().next() {
            match turn_state.current_phase {
                TurnPhase::Preparation => {
                    // 準備階段結束後轉入戰鬥階段
                    turn_state.current_phase = TurnPhase::Combat;
                }
                TurnPhase::Combat => {
                    // 戰鬥階段結束後轉入結算階段
                    turn_state.current_phase = TurnPhase::Resolution;
                }
                TurnPhase::Resolution => {
                    // 結算階段結束後，進入下一個玩家的準備階段
                    turn_state.current_phase = TurnPhase::Preparation;
                    turn_state.current_player = (turn_state.current_player + 1) % turn_state.total_players;
                    
                    // 如果回到第一個玩家，增加回合數
                    if turn_state.current_player == 0 {
                        turn_state.turn_number += 1;
                    }
                }
            }
        }
    }
}

// 回合管理器
pub struct TurnManager {
    preparation_time: f32,    // 準備階段時間（秒）
    combat_time: f32,        // 戰鬥階段時間（秒）
    resolution_time: f32,    // 結算階段時間（秒）
    current_time: f32,       // 當前階段已經過的時間
}

impl TurnManager {
    pub fn new(preparation_time: f32, combat_time: f32, resolution_time: f32) -> Self {
        Self {
            preparation_time,
            combat_time,
            resolution_time,
            current_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, turn_state: &mut TurnState) -> bool {
        self.current_time += delta_time;
        
        let phase_time = match turn_state.current_phase {
            TurnPhase::Preparation => self.preparation_time,
            TurnPhase::Combat => self.combat_time,
            TurnPhase::Resolution => self.resolution_time,
        };

        if self.current_time >= phase_time {
            self.current_time = 0.0;
            true  // 表示當前階段結束
        } else {
            false
        }
    }

    pub fn get_remaining_time(&self, turn_state: &TurnState) -> f32 {
        let phase_time = match turn_state.current_phase {
            TurnPhase::Preparation => self.preparation_time,
            TurnPhase::Combat => self.combat_time,
            TurnPhase::Resolution => self.resolution_time,
        };
        phase_time - self.current_time
    }
} 