use crate::*;

pub(crate) fn unordered_set_pagination<V>(
    set: &UnorderedSet<V>,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<V> where V: BorshSerialize + BorshDeserialize {

	let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
	assert_ne!(limit, 0, "Cannot provide limit of 0.");
	let start_index: u128 = from_index.map(From::from).unwrap_or_default();
	assert!(
		set.len() as u128 > start_index,
		"Out of bounds, please use a smaller from_index."
	);
	set
		.iter()
		.skip(start_index as usize)
		.take(limit)
		.map(|item| item)
		.collect()
}
	
/// from https://github.com/near/near-sdk-rs/blob/e4abb739ff953b06d718037aa1b8ab768db17348/near-contract-standards/src/non_fungible_token/utils.rs#L29

pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
	// log!("refund_deposit amount {}", refund);
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}