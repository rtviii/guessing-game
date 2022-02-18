use rand::Rng;
use std::vec;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg  ::{CurrentGameResponse, ExecuteMsg, InstantiateMsg, QueryMsg, Direction, CurrentParticipantsResponse};
use crate::state::{CURRENT_GAME_NUMBER,TOTALRECORD,GameState};

const CONTRACT_NAME: &str    = "crates.io:guessing-game-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // Have to set contract version
    // Create initial state
    // Respond

    let original_game_n = 0_u8;
    let state           = GameState {
        current_game_number: original_game_n,
        secret_number      : msg.originalHiddenNumber,
        last_attempt       : 0,
        participants       : vec![] 
    };

    CURRENT_GAME_NUMBER.save(deps.storage,&original_game_n)?;
    TOTALRECORD.save(deps.storage, "0", &state)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("secret_number","*")
        .add_attribute("games_played_so_far", original_game_n.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg : ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendGuess {guess}        => try_submit_guess(deps, info, guess),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCurrentGame {}  => to_binary(&query_current_game(deps)?),
        QueryMsg::GetParticipants {} => to_binary(&query_current_participants(deps)?),
    }
}

fn query_current_participants(deps: Deps) -> StdResult<CurrentParticipantsResponse> {
    let curn    = CURRENT_GAME_NUMBER.load(deps.storage).unwrap();
    let curgame = TOTALRECORD.load(deps.storage, &curn.to_string()).unwrap();

    Ok(CurrentParticipantsResponse{participants:curgame.participants})
}
fn query_current_game(deps: Deps) -> StdResult<CurrentGameResponse> {

    let cur_game_number = CURRENT_GAME_NUMBER.load(deps.storage).unwrap();
    let cur_game_state  = TOTALRECORD.load(deps.storage, &cur_game_number.to_string())?;
    
    Ok(CurrentGameResponse {
            current_game_number: cur_game_number,
            direction          : match cur_game_state.last_attempt < cur_game_state.secret_number {
                true  => Direction::Higher,
                false => Direction::Lower
            },
            last_guess: cur_game_state.last_attempt
            })
}


pub fn try_submit_guess(deps: DepsMut,info:MessageInfo, guess:u8) -> Result<Response, ContractError> {

    // update participants for current game
    // update last_attempt for current game

    // if guessed correctly: 
    // - generate a new random num 1-10
    // - instantiate new game
    // - update state

    // if guessed incorrectly:
    // - update last_attempt
    // - return the message with the tip to go higher or lower

    let curgamen = CURRENT_GAME_NUMBER.load(deps.storage).unwrap();
    TOTALRECORD.update(deps.storage, &curgamen.to_string(), |opt:Option<GameState>| -> Result<_,ContractError>{
        match  opt {
            Some(mut gamestate) =>{
                if !gamestate.participants.contains(&info.sender){
                    gamestate.participants.push(info.sender.clone());
                    gamestate.last_attempt = guess;
                }
                Ok(gamestate)
            },

            None => Err(ContractError::GameNotFound{
                gamen:  curgamen
            })
        }

    })?;

    let curgame  = TOTALRECORD.load(deps.storage, &curgamen.to_string()).unwrap();

    if curgame.secret_number == guess{

        let nextgame_n     = curgamen + 1;
        let mut rng        = rand::thread_rng();
        let nextgame_state = GameState {
            participants       : vec![],
            secret_number      : rng.gen_range(1..11),
            last_attempt       : 0,
            current_game_number: nextgame_n
        };

        CURRENT_GAME_NUMBER.save(deps.storage, &nextgame_n)?;
        TOTALRECORD.save(deps.storage, &nextgame_n.to_string(),&nextgame_state)?;

        return Ok(Response::new().add_attribute("direction", "you guessed correctly!"))
    }

    // update last guess
    let resp = Response::new().add_attribute("direction", match curgame.secret_number > guess {
        false => "go higher",
        true  => "go lower"
    });

    // build response
    // clean if guessed correctly
    Ok(resp)
}


// pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
//     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
//         if info.sender != state.owner {
//             return Err(ContractError::Unauthorized {});
//         }
//         state.count = count;
//         Ok(state)
//     })?;
//     Ok(Response::new().add_attribute("method", "reset"))
// }



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let mut rng = rand::thread_rng();
        let msg     = InstantiateMsg { originalHiddenNumber:rng.gen_range(1..11) };
        let info    = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        for msg in res.messages{
            println!("Response message: {:?}", &msg);
        }

        // it worked, let's query the state
        let res                        = query(deps.as_ref(), mock_env(), QueryMsg::GetCurrentGame {}).unwrap();
        let value: CurrentGameResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.current_game_number);

    }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies(&coins(2, "token"));

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies(&coins(2, "token"));

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    //     // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: CountResponse = from_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }
}
