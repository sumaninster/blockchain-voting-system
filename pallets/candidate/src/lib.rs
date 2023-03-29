#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_runtime::ArithmeticError;
use primitives::{ElectionId, CandidateId, CandidateInfo};
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
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ElectionInfo: ElectionInfo;/// Allowed origins for only election commission
		type ElectionCommissionApproveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn candidates)]
	// List of voters
	pub type Candidates<T: Config> = StorageDoubleMap<_, Blake2_128Concat, ElectionId, Blake2_128Concat, CandidateId, CandidateInfo>;

	#[pallet::storage]
	#[pallet::getter(fn election_id_counter)]
	/// CandidateId counter
	pub type CandidateIdCounter<T: Config> = StorageValue<_, CandidateId>;

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
		pub fn register_candidate(origin: OriginFor<T>, election_id: ElectionId, name: Vec<u8>, info: Vec<u8>) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			let id = Self::candidate_id_inc()?;
			ensure!(
				!&T::ElectionInfo::is_election_open_for_voter_registration(&election_id),
				Error::<T>::InvalidElectionIdOrNotOpenForRegistration
			);
			// Update storage for voter list for particular election id
			<Candidates<T>>::insert(election_id, id, CandidateInfo{
				id,
				name,
				info,
			});

			// Emit an event.
			Self::deposit_event(Event::VoterRegistered);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn candidate_id_inc() -> Result<CandidateId, DispatchError> {
		if CandidateIdCounter::<T>::get().is_some() {
			let candidate_id = CandidateIdCounter::<T>::get()
				.unwrap()
				.checked_add(1)
				.ok_or(ArithmeticError::Underflow)?;
			CandidateIdCounter::<T>::set(Option::from(candidate_id));
			Ok(candidate_id)
		} else {
			CandidateIdCounter::<T>::set(Some(1));
			Ok(1)
		}
	}
}
