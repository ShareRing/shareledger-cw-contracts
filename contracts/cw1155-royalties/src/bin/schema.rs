use cosmwasm_schema::write_api;
use cw1155_royalties::msg::{Cw1155RoyaltiesExecuteMsg, Cw1155RoyaltiesQueryMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: Cw1155RoyaltiesExecuteMsg,
        query: Cw1155RoyaltiesQueryMsg,
    }
}
