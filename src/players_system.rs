use specs::{System, ReadStorage, Join, Read};
use crate::turn::Player;
use crate::WebsocketChannel;
use std::collections::HashMap;

pub struct PlayersSystem;

impl<'a> System<'a> for PlayersSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        Read<'a, HashMap<String, WebsocketChannel>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, websocket_channels) = data;
    
        for player in (&players).join() {
            if let Some(channel) = websocket_channels.get(&player.name_id) {
                println!("Found channel for player {}: {:?}", player.name_id, channel);
            } else {
                println!("No channel found for player {}", player.name_id);
            }
        }
    }
}