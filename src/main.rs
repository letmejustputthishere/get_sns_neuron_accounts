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
    let icp_agent = Agent::builder()
        .with_url("https://icp0.io")
        .with_identity(AnonymousIdentity)
        .build()
        .expect("Failed to create agent");

    let fuel_agent = Agent::builder()
        .with_url("https://icp0.io")
        .with_identity(AnonymousIdentity)
        .build()
        .expect("Failed to create agent");

    // initialize the icrc agent
    let icp_ledger = Icrc1Agent {
        agent: icp_agent,
        ledger_canister_id: MAINNET_LEDGER_CANISTER_ID,
    };

    let fuel_ledger = Icrc1Agent {
        agent: fuel_agent,
        ledger_canister_id: Principal::from_str("nfjys-2iaaa-aaaaq-aaena-cai").unwrap(),
    };

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

    let balance = fuel_ledger
        .balance_of(
            Account {
                owner: sns_governance_principal,
                subaccount: Some(subaccount),
            },
            CallMode::Update,
        )
        .await
        .expect("Failed to get balance");
    println!("Treasury Fuel balance: {}", balance);
}
