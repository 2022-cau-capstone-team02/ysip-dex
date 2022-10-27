use cosmwasm_std::coin;
use cw20::{BalanceResponse, Cw20QueryMsg};
use testing::execute::{execute_mint, execute_provide_liquidity, execute_swap};
use testing::init::{mock_cw20_contract, mock_pair_contract};
use testing::instantiate::{instantiate_cw20_contract, instantiate_pair_contract};
use testing_base::consts::ADDR1;
use testing_base::init::init_app;

fn main() {
    let mut app = init_app(ADDR1);
    let channel_a_code_id = app.store_code(mock_cw20_contract());
    let pair_code_id = app.store_code(mock_pair_contract());


    let channel_a_contract_addr = instantiate_cw20_contract(
        &mut app,
        channel_a_code_id,
        &[coin(1000, "ukrw")],
        ADDR1,
        ADDR1,
        "channel_a",
        "channel-a",
        vec![],
        "channel_a",
    );

    let _mint_res = execute_mint(
        &mut app,
        channel_a_contract_addr.as_ref(),
        ADDR1,
        ADDR1,
        100,
    );

    let pair_contract_addr = instantiate_pair_contract(
        &mut app,
        pair_code_id,
        channel_a_code_id,
        &[coin(1000, "ukrw")],
        ADDR1,
        ADDR1,
        channel_a_contract_addr.as_ref(),
        "ukrw",
        "pair",
    );

    let liquidity_res = execute_provide_liquidity(
        &mut app,
        "ukrw",
        channel_a_contract_addr.as_ref(),
        pair_contract_addr.as_ref(),
        ADDR1,
    );

    println!("{:?}", liquidity_res);

    let balance: BalanceResponse = app.wrap().query_wasm_smart(
        channel_a_contract_addr,
        &Cw20QueryMsg::Balance {
            address: ADDR1.to_string()
        }).unwrap();

    println!("{:?}", balance);

    // let swap_res = execute_swap(
    //     &mut app,
    //     pair_contract_addr.as_ref(),
    //     channel_a_contract_addr.as_ref(),
    //     30
    // );
}