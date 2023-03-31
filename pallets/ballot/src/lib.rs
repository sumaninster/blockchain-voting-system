#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use frame_support::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_system::pallet_prelude::*;
use primitives::{ElectionId, CandidateId, VoteCount, ZKPCommitmentValue};//, Vote, VotingSystem};
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
		type ElectionInfo: ElectionInfo;
		/// Allowed origins for only election commission
		type ElectionCommissionApproveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn ballot)]
	// Votes for candiates
	pub type Ballot<T: Config> = StorageDoubleMap<_, Blake2_128Concat, ElectionId, Blake2_128Concat, CandidateId, VoteCount>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Vote is successfully casted
		/// parameters. []
		VoteCasted,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error: Invalid Election Id Or Not Open For Voting.
		InvalidElectionIdOrNotOpenForVoting,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Self register by voter
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn cast_vote(origin: OriginFor<T>, election_id: ElectionId, candidate_id: CandidateId, _zkp_value: ZKPCommitmentValue, _transcript: Vec<u8>, _zkp_randomness: Vec<u8>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(
				!&T::ElectionInfo::is_election_open_for_voting(&election_id),
				Error::<T>::InvalidElectionIdOrNotOpenForVoting
			);
			
			//let transcript_ref: &'static [u8] = unsafe { std::mem::transmute(transcript.as_slice()) };

			/*let vs = VotingSystem::new(transcript_ref);
	        let vote = vs.cast_vote(
	            candidate_id, 
	            zkp_value, 
	            &zkp_randomness, 
	        );*/
			// Update storage for vote count for particular election id and candidate id
			let _ = Self::vote_inc(election_id, candidate_id);

			// Emit an event.
			Self::deposit_event(Event::VoteCasted);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn vote_inc(election_id: ElectionId, candidate_id: CandidateId) -> DispatchResult {
		if <Ballot<T>>::contains_key(election_id, candidate_id) {
				<Ballot<T>>::mutate(
					election_id,
					candidate_id,
					|count| -> DispatchResult {
						if let Some(c) = count {
							*count = Option::from(*c+1);
						} else {
							*count = Option::from(1);
						}
						Ok(())
					},
				)
			} else {
				<Ballot<T>>::insert(election_id, candidate_id, 1);
				Ok(())
			}
	}
}
