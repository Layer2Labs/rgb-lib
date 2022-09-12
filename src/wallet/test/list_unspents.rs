use super::*;

#[test]
fn success() {
    initialize();

    // no unspents
    let (wallet, _online) = get_empty_wallet!();
    let unspent_list = wallet.list_unspents(true).unwrap();
    assert_eq!(unspent_list.len(), 0);

    // one (settled) unspent, no rgb allocations
    let (wallet, _online) = get_funded_noutxo_wallet!();
    wallet._sync_db_txos().unwrap();
    let unspent_list = wallet.list_unspents(true).unwrap();
    assert_eq!(unspent_list.len(), 1);
    let unspent_list_all = wallet.list_unspents(false).unwrap();
    assert_eq!(unspent_list_all.len(), 1);

    // more unspents, one with an rgb allocation
    let (mut wallet, online) = get_funded_wallet!();
    let asset = wallet
        .issue_asset(
            online.clone(),
            TICKER.to_string(),
            NAME.to_string(),
            PRECISION,
            AMOUNT,
        )
        .unwrap();
    let unspent_list = wallet.list_unspents(true).unwrap();
    assert_eq!(unspent_list.len(), UTXO_NUM as usize + 1);
    let unspent_list_all = wallet.list_unspents(false).unwrap();
    assert_eq!(unspent_list_all.len(), UTXO_NUM as usize + 1);
    let unspents_with_rgb_allocations: Vec<Unspent> = unspent_list
        .into_iter()
        .filter(|u| u.rgb_allocations.len() > 0)
        .collect();
    assert!(unspents_with_rgb_allocations.len() == 1);
    let allocation_asset_ids: Vec<String> = unspents_with_rgb_allocations
        .first()
        .unwrap()
        .rgb_allocations
        .clone()
        .into_iter()
        .map(|a| a.asset_id.unwrap_or(s!("")))
        .collect();
    assert!(allocation_asset_ids.contains(&asset.asset_id));
}