#[cfg(test)] // allows us to compile code, based on the "test" flag.

use super::*;
use support::{impl_outer_origin};
use runtime_io::with_externalities;
use primitives::{H256, Blake2Hasher}; //called substrate_primitives as primitives
use support::{assert_ok, assert_noop};
use runtime_primitives::{
    BuildStorage,
    traits::{IdentityLookup, BlakeTwo256}, // Test wrapper for this specific type/ looks up the identity; returns Result
    testing::{Digest, DigestItem, Header}
};

// impl outer origin
impl_outer_origin! {
    pub enum Origin for Test {}
}

// For testing the module, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl system::Trait for Test {
	// We are just aliasing the types with the type, or some easier abstration!!
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Digest = Digest;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type Log = DigestItem;
}

// code above inherits but still have to declare it in test
impl balances::Trait for Test {
	type Balance = u64;			// aliasing u64 as "balance" to mock the balance
	type OnFreeBalanceZero = ();
	type OnNewAccount = ();
	type Event = ();
	type TransactionPayment = ();
	type TransferPayment = ();
	type DustRemoval = ();
}

impl timestamp::Trait for Test {
	type Moment = u64;
	type OnTimestampSet = ();
}

impl erc721::Trait for Test{
	type Event = ();
	// type Currency = Balance;
}

// this module, implements the traits.
impl Trait for Test {
	type Event = ();
	type Currency = balances::Module<Test>;
	// any custom traits from this module?
}

// Alias the module names for easy usage
type Debt = Module<Test>;
type Balance = balances::Module<Test>;
type Timestamp = timestamp::Module<Test>;
type ERC = erc721::Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
	let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
	t.extend(balances::GenesisConfig::<Test>{
		balances: vec![(0, 100),(1, 100),(2, 100)], //initializes some accts with balances
		transaction_base_fee: 0,
		transaction_byte_fee: 0,
		transfer_fee: 0,
		creation_fee: 0,
		existential_deposit: 0,
		vesting: vec![],
	}.build_storage().unwrap().0);
	// last step... what this do?
	t.into()
}

// UNIT Tests
#[test]
fn should_create_debt_request() {
	with_externalities(&mut new_test_ext(), || {
		//       uses the Alias
		assert_ok!(Debt::borrow(Origin::signed(0), 0, 1, 100, 0, 0, 1));

		// Timestamp hasn't incremented, so hash should stay the time
		assert_noop!(Debt::borrow( Origin::signed(0), 0, 1, 100, 0, 0, 1),
		"Error: Debt request already exists");
	});
}

#[test]
fn should_fulfill_request() {
	with_externalities(&mut new_test_ext(), || {
		// set up
		ERC::create_token(Origin::signed(0));
    let token_id = ERC::token_by_index(0);

		//       uses the Alias
		assert_ok!(Debt::borrow(Origin::signed(0), 0, 1, 100, 0, 0, 1));
		let debt_id = Debt::get_debt_id(0);

		// Debt isn't collateralized yet
		assert!(Debt::fulfill(Origin::signed(1), debt_id).is_err());
		
		// should be able to fulfill debt
		assert_ok!(ERC::collateralize_token(Origin::signed(0), token_id, debt_id));
		assert!(Debt::fulfill(Origin::signed(1), debt_id).is_ok());
		assert_eq!(0, Balance::free_balance(&1));
		assert_eq!(200, Balance::free_balance(&0));
	
	
		// Check debt now has creditor


		// should fail if is already issued
		// 3rd person cannot fulfill debt... bc creditor exists now.
		
	});
}

// #[test]
// fn should_collateralize() {
// 	with_externalities(&mut new_test_ext(), || { 
// 		Collateral::create_debt_request(Origin::signed(0), 5, 1, 12345);
// 		let debtor = Origin::signed(0);
// 		// outer call.      inner call                   dispatch
// 		// owner needs to approve

// 		let request_id = Collateral::get_debt_request_id(0);
// 		// let token_id = erc721::token_by_index(0); // grab the first and only token

// 		// assert_ok!(Collateral::collateralize_debt_request(debtor, token_id, request_id ));
// 	});
// }	

