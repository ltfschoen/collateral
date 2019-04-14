/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use super::erc721;
use parity_codec::{Encode, Decode}; //enables #[derive(Decode)] Why? what is it
use runtime_primitives::traits::{Hash, StaticLookup}; // Zero, As //static look up is for beneficiary address

#[cfg(test)]
mod test;

/// The module's configuration trait.
pub trait Trait: timestamp::Trait + erc721::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq)] //Encode, Deco req for enums, #[cfg_attr(feature = "std", derive(Debug))]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum Status {
	Open, 			// looking for issuance
	Active, 		// loan issued, pending payment
	Expired,		/// loan never filled, expired
	Repaid, 		// closed, repaid
	Defaulted,		// unpaid, collat seized
}

// Status is by default
impl Default for Status {
	fn default() -> Self { Status::Open }
}

// Asset owners can create a DebtRequest to ask for a traunche of Balance
#[derive(Encode, Decode, Default, Clone, PartialEq)] //these are custom traits required by all structs (some traits forenums)
#[cfg_attr(feature = "std", derive(Debug))] // attr provided by rust compiler. uses derive(debug) trait when in std mode
pub struct Debt<AccountId, Balance, Moment, Hash> {   //Needs the blake2 Hash trait
	requestor: AccountId,		// Account that will go in debt
	beneficiary: AccountId,	// Recipient of Balance
	amount: Balance,				// Principal loan
	expiry: Moment,					// Duration of debtRequest
	status: Status,					// Default is open
	// Filled in after loan is fulfilled by someone
	collateral: Hash,		// ID of the collateral for this loan
	creditor: AccountId,  	// null as default
}

type DebtIndex = u64; //like proposalindex in treasury

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Debt {
				// TODO later abstrate T::Hash into generic vars, so its not so long?
		// doesn't get deleted
		Debts get(get_debt_order): map T::Hash => Debt<T::AccountId, T::Balance, T::Moment>; //DebtRequest ID to the RequestItself
		// [0, 0x...] [1, 0x...]
		DebtIndexToId get(get_debt_id): map DebtIndex => T::Hash;
		DebtCount get(get_total_debts): DebtIndex;  //Alias for u64
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		pub fn borrow(
				origin, 
				amount: T::Balance, 
				beneficiary: <T::Lookup as StaticLookup>::Source, 
				expiry: T::Moment
		) { //TODO, change expiry
			let requestor = ensure_signed(origin)?;		//macro, returns sender address
			let now = <timestamp::Module<T>>::get();

			// Q: whats the diff btw this and just doing <t as system:: trait> .. etc.
			let debt_id = (<system::Module<T>>::random_seed(), &requestor, now).using_encoded(<T as system::Trait>::Hashing::hash); // use runtime_primitives::hash, its a constnat!
			let beneficiary = T::Lookup::lookup(beneficiary)?;		//looks up the accountId.

			// TODO make sure debtrequest doesn't exist already, in case they try to overwrite debt..
			ensure!(!<Debts<T>>::exists(debt_id), "Error: Debt request already exists");
			let new_debt = Debt {
				requestor: requestor.clone(),
				beneficiary: beneficiary.clone(),
				amount,
				expiry
			};

			// Add new debt request to DebtRequests map
			let i = Self::get_total_debt_requests();
			<DebtCount<T>>::put(i+1); //increment total count by 1
			<DebtIndexToId<T>>::insert(i, debt_id);
			<Debts<T>>::insert(debt_id, new_debt);
			// Emit the event

			Self::deposit_event(RawEvent::DebtCreated(requestor, debt_id));
			// TODO remove later
		}

		// TODO: check if debt has been collateralized. 
		// on_initialize
		// 
	}
}


decl_event!(
	pub enum Event<T> where 
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
	{
		// 								debtor, requestId
		DebtCreated(AccountId, Hash),
	}
);
