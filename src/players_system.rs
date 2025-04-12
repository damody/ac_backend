use specs::{System, ReadStorage, Join, Read, Write};
use crate::turn::Player;
use crate::WebsocketChannel;
use std::collections::HashMap;

pub struct PlayersSystem;

impl<'a> System<'a> for PlayersSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        Write<'a, HashMap<String, WebsocketChannel>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, mut websocket_channels) = data;
    
        for player in (&players).join() {
            if let Some(channel) = websocket_channels.get_mut(&player.name_id) {
                if let Ok(message) = channel.rx_from_websocket.try_recv() {
                    println!("Received message for player {}: {:?}", player.name_id, message);
                }
            } else {
                println!("No channel found for player {}", player.name_id);
            }
        }
    }
}