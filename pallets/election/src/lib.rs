#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_runtime::ArithmeticError;
use primitives::ElectionId;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

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
		/// Allowed origins for only election commission
		type ElectionCommissionApproveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/

	#[pallet::storage]
	#[pallet::getter(fn open_for_voter_registration)]
	// Active elections that are open for registration
	pub type ElectionOpenForVoterRegistration<T> = StorageMap<_, Blake2_128Concat, ElectionId, bool>;

	#[pallet::storage]
	#[pallet::getter(fn open_for_election)]
	// Active elections that are open for registration
	pub type ElectionOpenForVoting<T> = StorageMap<_, Blake2_128Concat, ElectionId, bool>;

	#[pallet::storage]
	#[pallet::getter(fn election_complete)]
	// Active elections that are open for registration
	pub type ElectionComplete<T> = StorageMap<_, Blake2_128Concat, ElectionId, bool>;

	#[pallet::storage]
	#[pallet::getter(fn election_id_counter)]
	/// ElectionId counter
	pub type ElectionIdCounter<T: Config> = StorageValue<_, ElectionId>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Election id is successfully registered
		/// parameters. []
		ElectionIdRegistered{election_id: ElectionId},
		/// Election id is successfully de-registered
		/// parameters. []
		ElectionIdDeRegistered,
		/// Election id is successfully activated
		/// parameters. []
		ElectionIdActivated,
		/// Election id is successfully de-activated
		/// parameters. []
		ElectionIdDeActivated,
		/// Election id is registered For Voting
		/// parameters. []
		ElectionIdRegisteredForVoting{election_id: ElectionId},
		/// Election id is de-registered For Voting
		/// parameters. []
		ElectionIdDeRegisteredForVoting,
		/// Election id is Open For Voting
		/// parameters. []
		ElectionIdOpenForVoting{election_id: ElectionId},
		/// Election id is Close For Voting
		/// parameters. []
		ElectionIdCloseForVoting
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error: Invalid Election Id.
		InvalidElectionId,
		/// Error: Election Id Already Open For Registration.
		ElectionIdAlreadyOpenForVoterRegistration,
		/// Error: Election Id Not Open For Voter Registration.
		ElectionIdNotOpenForVoterRegistration,
		/// Error: Election Id Already Open For Voting.
		ElectionIdAlreadyOpenForVoting,
		/// Error: Election Id Already not Open For Voting.
		ElectionIdNotOpenForVoting,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Self register by voter
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_new_election_id(origin: OriginFor<T>) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			let election_id = Self::election_id_inc()?;
			// Update storage for election id
			<ElectionOpenForVoterRegistration<T>>::insert(election_id, false);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdRegistered{election_id});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// Self register by voter
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn de_register_election_id(origin: OriginFor<T>, election_id: ElectionId) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			// Update storage for election id
			<ElectionOpenForVoterRegistration<T>>::remove(election_id);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdDeRegistered);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// Self register by voter
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn election_open_for_voter_registration(origin: OriginFor<T>, election_id: ElectionId) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			ensure!(
				!<ElectionOpenForVoterRegistration<T>>::contains_key(&election_id),
				Error::<T>::InvalidElectionId
			);
			ensure!(
				<ElectionOpenForVoterRegistration<T>>::get(&election_id).unwrap_or(false),
				Error::<T>::ElectionIdAlreadyOpenForVoterRegistration
			);
			// Update storage for election id
			<ElectionOpenForVoterRegistration<T>>::insert(election_id, true);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdActivated);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// Self register by voter
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn election_close_for_voter_registration(origin: OriginFor<T>, election_id: ElectionId) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			ensure!(
				!<ElectionOpenForVoterRegistration<T>>::contains_key(&election_id),
				Error::<T>::InvalidElectionId
			);
			ensure!(
				!<ElectionOpenForVoterRegistration<T>>::get(&election_id).unwrap_or(false),
				Error::<T>::ElectionIdNotOpenForVoterRegistration
			);
			// Update storage for election id
			<ElectionOpenForVoterRegistration<T>>::insert(election_id, false);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdDeActivated);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// Self register by voter
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_election_for_voting(origin: OriginFor<T>) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			Self::election_id_inc()?;
			let election_id = ElectionIdCounter::<T>::get().expect("Pool Id not found");
			// Update storage for election id
			<ElectionOpenForVoting<T>>::insert(election_id, false);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdRegisteredForVoting{election_id});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// Self register by voter
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn de_register_election_for_voting(origin: OriginFor<T>, election_id: ElectionId) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			// Update storage for election id
			<ElectionOpenForVoting<T>>::remove(election_id);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdDeRegisteredForVoting);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// Self register by voter
		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn election_open_for_voting(origin: OriginFor<T>, election_id: ElectionId) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			ensure!(
				!<ElectionOpenForVoting<T>>::contains_key(&election_id),
				Error::<T>::InvalidElectionId
			);
			ensure!(
				<ElectionOpenForVoting<T>>::get(&election_id).unwrap_or(false),
				Error::<T>::ElectionIdAlreadyOpenForVoting
			);
			// Update storage for election id
			<ElectionOpenForVoting<T>>::insert(election_id, true);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdOpenForVoting{election_id});
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		/// Self register by voter
		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn election_close_for_voting(origin: OriginFor<T>, election_id: ElectionId) -> DispatchResult {
			// Check if the sender is an approved origin or not
			T::ElectionCommissionApproveOrigin::ensure_origin(origin)?;
			ensure!(
				!<ElectionOpenForVoting<T>>::contains_key(&election_id),
				Error::<T>::InvalidElectionId
			);
			ensure!(
				!<ElectionOpenForVoting<T>>::get(&election_id).unwrap_or(false),
				Error::<T>::ElectionIdNotOpenForVoting
			);
			// Update storage for election id
			<ElectionOpenForVoting<T>>::insert(election_id, false);

			// Emit an event.
			Self::deposit_event(Event::ElectionIdCloseForVoting);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn election_id_inc() -> Result<ElectionId, DispatchError> {
		if ElectionIdCounter::<T>::get().is_some() {
			let election_id = ElectionIdCounter::<T>::get()
				.unwrap()
				.checked_add(1)
				.ok_or(ArithmeticError::Underflow)?;
			ElectionIdCounter::<T>::set(Option::from(election_id));
			Ok(election_id)
		} else {
			ElectionIdCounter::<T>::set(Some(1));
			Ok(1)
		}
	}
}

impl<T: Config> ElectionInfo for Pallet<T> {
	fn is_election_open_for_voter_registration(election_id: &ElectionId) -> bool {
		<ElectionOpenForVoterRegistration<T>>::get(election_id).unwrap_or(false)
	}
	fn is_election_open_for_voting(election_id: &ElectionId) -> bool {
		<ElectionOpenForVoting<T>>::get(election_id).unwrap_or(false)
	}
}

pub trait ElectionInfo {
	fn is_election_open_for_voter_registration(election_id: &ElectionId) -> bool;
	fn is_election_open_for_voting(election_id: &ElectionId) -> bool;
}
