use std::env::current_dir;
use std::fs::{create_dir_all};

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use guessing_game_contract::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg,CurrentGameResponse,CurrentParticipantsResponse,Direction};
use guessing_game_contract::state::GameState;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");

    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(GameState), &out_dir);
    export_schema(&schema_for!(CurrentGameResponse), &out_dir);
    export_schema(&schema_for!(CurrentParticipantsResponse), &out_dir);
    export_schema(&schema_for!(Direction), &out_dir);
}
