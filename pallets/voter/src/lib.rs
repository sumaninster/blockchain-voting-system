#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_system::pallet_prelude::*;
use primitives::ElectionId;
use pallet_election::ElectionInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ElectionInfo: ElectionInfo;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn voters)]
	// List of voters
	pub type Voters<T> = StorageMap<_, Blake2_128Concat, ElectionId, AccountIdOf<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Voter is successfully registered
		/// parameters. []
		VoterRegistered,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error: Invalid Election Id Or Not Open For Registration.
		InvalidElectionIdOrNotOpenForRegistration,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Self register by voter
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_voter(origin: OriginFor<T>, election_id: ElectionId, _key: Vec<u8>, _value: Vec<u8>) -> DispatchResult {
			let voter_account = ensure_signed(origin)?;
			ensure!(
				!&T::ElectionInfo::is_election_open_for_voter_registration(&election_id),
				Error::<T>::InvalidElectionIdOrNotOpenForRegistration
			);
			// Update storage for voter list for particular election id
			<Voters<T>>::insert(election_id, voter_account);

			// Emit an event.
			Self::deposit_event(Event::VoterRegistered);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
