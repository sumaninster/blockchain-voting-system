#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use scale_info::{TypeInfo};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use frame_support::inherent::Vec;
//use curve25519_dalek_ng::{scalar::Scalar, ristretto::CompressedRistretto};
//use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};

//pub mod zkp;

/// The type for identifying the elections
pub type ElectionId = u64;
/// The type for identifying the canditates
pub type CandidateId = u64;
/// The type for identifying the ZKP Commitment Value
pub type ZKPCommitmentValue = u64;
/// The type for identifying the ZKP Commitment Value
pub type VoteCount = u128;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CandidateInfo {
	pub id: CandidateId,
	pub name: Vec<u8>,
	pub info: Vec<u8>,
}
/*
//Implement the ZKP Module:
pub struct ZKPModule {
    pc_gens: PedersenGens,
    bp_gens: BulletproofGens,
}

#[derive(Clone, RuntimeDebug)]
pub struct PedersenCommitment {
    pub value: u64,
    pub randomness: Scalar,
}

// Implement the Voting System:
#[derive(Clone, RuntimeDebug)]
pub struct Vote {
    pub candidate: CandidateId,
    pub commitment: PedersenCommitment,
    pub proof: RangeProof,
    pub committed_value: CompressedRistretto,
}

pub struct VotingSystem {
    pub transcript: &'static [u8],
}*/