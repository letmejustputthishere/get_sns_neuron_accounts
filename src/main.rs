use std::str::FromStr;

use dfn_core::api::PrincipalId;
use ic_nervous_system_common::ledger::compute_distribution_subaccount;
use icp_ledger::AccountIdentifier;
fn main() {
    // SNS governance canister id
    let principal = PrincipalId::from_str("nmkto-maaaa-aaaaq-aaemq-cai").unwrap();

    // calculates the subaccount where the SNS governance canister
    // holds the SNS token treasury on the SNS ledger.
    // this is not the default subaccount, as this is the minter account,
    // thus any incoming transfers would be considered burns and any
    // outgoing transfers would be considered mints.
    let subaccount = compute_distribution_subaccount(principal, 0);
    println!("SNS treasury subaccount: {}", subaccount);

    let account_id = AccountIdentifier::new(
        PrincipalId::from_str("nmkto-maaaa-aaaaq-aaemq-cai").unwrap(),
        Some(subaccount),
    );
    println!("account-id: {}", account_id)
}
