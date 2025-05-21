use hex::FromHex;
use std::str::FromStr;

use dfn_core::api::PrincipalId;
use ic_agent::{Agent, export::Principal, identity::AnonymousIdentity};
use ic_ledger_types::MAINNET_LEDGER_CANISTER_ID;
use ic_nervous_system_common::ledger::compute_distribution_subaccount_bytes;
use icrc_ledger_agent::{CallMode, Icrc1Agent};
use icrc_ledger_types::icrc1::account::Account;

#[tokio::main]
async fn main() {
    // SNS governance canister id
    let prinicpal_string = "nmkto-maaaa-aaaaq-aaemq-cai";
    let sns_governance_principal = Principal::from_str(prinicpal_string).unwrap();

    // initialize the agents
    let agent = Agent::builder()
        .with_url("https://icp0.io")
        .with_identity(AnonymousIdentity)
        .build()
        .expect("Failed to create agent");

    // initialize the icrc agent
    let icp_ledger = Icrc1Agent {
        agent: agent.clone(),
        ledger_canister_id: MAINNET_LEDGER_CANISTER_ID,
    };

    let fuel_ledger = Icrc1Agent {
        agent: agent,
        ledger_canister_id: Principal::from_str("nfjys-2iaaa-aaaaq-aaena-cai").unwrap(),
    };

    // the icp of the treasury is held by the default account of the sns governance canister
    let balance = icp_ledger
        .balance_of(
            Account {
                owner: sns_governance_principal,
                subaccount: None,
            },
            CallMode::Update,
        )
        .await
        .expect("Failed to get balance");
    println!("Treasury ICP balance: {}", balance);

    // calculates the subaccount where the SNS governance canister
    // holds the SNS token treasury on the SNS ledger.
    // this is not the default subaccount, as this is the minter account,
    // thus any incoming transfers would be considered burns and any
    // outgoing transfers would be considered mints.
    let subaccount =
        compute_distribution_subaccount_bytes(PrincipalId::from_str(prinicpal_string).unwrap(), 0);
    // let subaccount = compute_distribution_subaccount(sns_governance_canister_id, 0);
    // println!("SNS treasury subaccount: {}", subaccount);

    let treasury_account = Account {
        owner: sns_governance_principal,
        subaccount: Some(subaccount),
    };
    println!("fuel token treasury account: {treasury_account}");

    let balance = fuel_ledger
        .balance_of(treasury_account, CallMode::Update)
        .await
        .expect("Failed to get balance");
    println!("Treasury Fuel balance: {}", balance);

    // the subaccount of the fuel governance canister on the fuel ledger belonging
    // to a specific fuel neuron is equivalent to the neuron id.
    // the below neuron id / subaccount was obtained by calling list_neurons on the governance
    // canister with the below developer neuron principal as an arugment:
    // 45uha-nnefe-yiih4-lcjg5-q3mm6-c2xsg-7xwxi-v37db-oliaz-qmovh-eqe
    let neuron_account = Account {
        owner: sns_governance_principal,
        subaccount: Some(
            <[u8; 32]>::from_hex(
                "30b259a09af47f31b200c91be926eae52e33719a79ebc382bb4b5926bf662063",
            )
            .expect("Decoding failed"),
        ),
    };
    println!("{}", neuron_account)
    
    // we can use list_community_fund_participants to validate the neuron fund participants
    // https://dashboard.internetcomputer.org/canister/nci6g-xqaaa-aaaaq-aaenq-cai#list_community_fund_participants
    // should be 64 participants contributing 8_816_968_789_320 ICP, as according to get_derived_state

    // we can call list_direct_participants to validate direct participation
    // https://dashboard.internetcomputer.org/canister/nci6g-xqaaa-aaaaq-aaenq-cai#list_direct_participants
    // should be 372 participants contributing 20_539_941_559_379 ICP, according to get_derived_state
}
