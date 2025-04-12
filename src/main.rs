#![allow(warnings)]
mod game_state;
mod combat;
mod turn;
mod skills;
mod players_system;

use specs::{Component, VecStorage, World, WorldExt, Builder, System, ReadStorage, WriteStorage, Join};
use specs::prelude::*;
use serde::{Serialize, Deserialize};
use futures_util::SinkExt;
use futures_util::StreamExt;
use uuid::Uuid;
use std::time::{Instant, Duration};
use tokio::sync::mpsc;
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
    name: String,        // 棋子的名字
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

#[derive(Debug)]
pub enum ChannelMessage {
    WebSocketEvent(String), // Example: Message from WebSocket
    SpecsEvent(String),     // Example: Message from Specs
}

// 創建兩個 channel
use std::collections::HashMap;
#[derive(Debug)]
pub struct SpecsChannel {
    pub tx_to_specs: mpsc::Sender<ChannelMessage>,
    pub rx_from_specs: mpsc::Receiver<ChannelMessage>,
}

impl Clone for SpecsChannel {
    fn clone(&self) -> Self {
        SpecsChannel {
            tx_to_specs: self.tx_to_specs.clone(),
            rx_from_specs: panic!("Receiver cannot be cloned"),
        }
    }
}
#[derive(Debug)]
pub struct WebsocketChannel {
    pub tx_to_websocket: mpsc::Sender<ChannelMessage>,
    pub rx_from_websocket: mpsc::Receiver<ChannelMessage>,
}


#[tokio::main]
async fn main() {
    env_logger::init();
    
    
    let mut websocket_player_channels: HashMap<String, WebsocketChannel> = HashMap::new();
    let mut specs_player_channels: HashMap<String, SpecsChannel> = HashMap::new();

    // 初始化每個玩家的通道
    for i in 0..4 {
        let name_id = format!("player_{}", i);
        let (tx_to_specs, rx_from_websocket) = mpsc::channel::<ChannelMessage>(32);
        let (tx_to_websocket, rx_from_specs) = mpsc::channel::<ChannelMessage>(32);
        websocket_player_channels.insert(
            name_id.clone(),
            WebsocketChannel {
                tx_to_websocket: tx_to_websocket,
                rx_from_websocket: rx_from_websocket,
            },
        );
        
        specs_player_channels.insert(
            name_id.clone(),
            SpecsChannel {
                tx_to_specs: tx_to_specs,
                rx_from_specs: rx_from_specs,
            },
        );
    }

    // 啟動 WebSocket 伺服器
    tokio::spawn(async move {
        let addr = "127.0.0.1:8080";
        let listener = tokio::net::TcpListener::bind(addr).await.expect("無法綁定 WebSocket 伺服器");
        println!("WebSocket 伺服器正在監聽 {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let specs_player_channels = specs_player_channels.clone(); // Use Arc to share across threads
            tokio::spawn(async move {
                let ws_stream = tokio_tungstenite::accept_async(stream).await.expect("無法接受 WebSocket 連線");
                println!("新的 WebSocket 連線已建立");
        
                let (mut write, mut read) = ws_stream.split();
        
                // Wait for the first message to extract the player's name
                if let Some(Ok(msg)) = read.next().await {
                    if let Ok(text) = msg.to_text() {
                        println!("收到初始訊息: {}", text);
        
                        // Parse JSON to extract the player's name
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(text) {
                            if let Some(name) = parsed.get("name").and_then(|n| n.as_str()) {
                                    let specs_channels = specs_player_channels;
                                    if let Some(channel) = specs_channels.get(name) {
                                    let tx_to_specs = channel.tx_to_specs.clone();
        
                                    // Process subsequent messages
                                    while let Some(Ok(msg)) = read.next().await {
                                        if let Ok(text) = msg.to_text() {
                                            println!("收到訊息: {}", text);
        
                                            // Forward WebSocket message to specs via tx_to_specs
                                            if let Err(e) = tx_to_specs.send(ChannelMessage::WebSocketEvent(text.to_string())).await {
                                                eprintln!("Failed to send message to specs: {}", e);
                                            }
                                        }
                                    }
                                } else {
                                    eprintln!("No channel found for name: {}", name);
                                }
                            } else {
                                eprintln!("Invalid JSON format: missing 'name' field");
                            }
                        } else {
                            eprintln!("Failed to parse JSON: {}", text);
                        }
                    }
                }
            });
        }
    });

    // 創建遊戲狀態
    let mut game_state = game_state::GameState::new(websocket_player_channels);
    
    // 初始化遊戲（設置4個玩家）
    game_state.initialize_game(4);
    
    // 創建分發器
    let mut dispatcher = DispatcherBuilder::new()
        .with(combat::CombatSystem, "combat_system", &[])
        .with(turn::TurnSystem, "turn_system", &["combat_system"])
        .with(players_system::PlayersSystem, "players_system", &["turn_system"])
        .build();

    // 初始化分發器
    dispatcher.setup(&mut game_state.world);

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

