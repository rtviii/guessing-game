use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
    pub current_game_number: u8,
    pub secret_number      : u8,
    pub last_attempt       : u8,
    pub participants       : Vec<Addr>
}

// game number -> that game's state
pub const TOTALRECORD: Map<&str,  GameState> = Map::new("total_record");
pub const CURRENT_GAME_NUMBER : Item<u8> = Item::new("current_game_number");
