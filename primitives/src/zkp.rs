use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use merlin::Transcript;
use curve25519_dalek_ng::{scalar::Scalar, ristretto::CompressedRistretto};
use sha2::{Digest, Sha512};
use crate::{CandidateId, ZKPCommitmentValue, PedersenCommitment, ZKPModule, Vote, VotingSystem};

// Define the ZKP Protocol:
// https://doc-internal.dalek.rs/bulletproofs/struct.RangeProof.html

impl PedersenCommitment {
    pub fn new(value: u64, randomness: Scalar) -> PedersenCommitment {
        PedersenCommitment { value, randomness }
    }

    pub fn generate_proof_of_knowledge(
        &self,
        pc_gens: &PedersenGens,
        bp_gens: &BulletproofGens,
        mut prover_transcript: Transcript,
    ) -> Option<(RangeProof, CompressedRistretto)> {
        let _com_randomness =
            pc_gens.commit(Scalar::from(self.value), self.randomness);

        let (proof, committed_value) = RangeProof::prove_single(
            &bp_gens,
            &pc_gens,
            &mut prover_transcript,
            self.value,
            &self.randomness,
            64,
        )
        .expect("Failed to generate range proof");

        // Verification requires a transcript with identical initial state:
        let mut verifier_transcript = Transcript::new(b"ZKPExample");
        if proof.verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &committed_value, 64).is_ok() {
            Some((proof, committed_value))
        } else {
            None
        }
    }

    pub fn verify_proof_of_knowledge(
        &self,
        pc_gens: &PedersenGens,
        bp_gens: &BulletproofGens,
        proof: &RangeProof,
        mut verifier_transcript: Transcript,
        committed_value: &CompressedRistretto,
    ) -> bool {
        // Verify the range proof using the Pedersen commitment generators and bulletproof generators
        proof.verify_single(&bp_gens, &pc_gens, &mut verifier_transcript, &committed_value, 64).is_ok()
    }
}

impl ZKPModule {
    pub fn new() -> ZKPModule {
        ZKPModule {
            pc_gens: PedersenGens::default(),
            bp_gens: BulletproofGens::new(64, 1),
        }
    }

    pub fn generate_commitment(value: u64, randomness: Scalar) -> PedersenCommitment {
        PedersenCommitment::new(value, randomness)
    }

    pub fn generate_proof_of_knowledge(
        &self,
        commitment: &PedersenCommitment,
        prover_transcript: Transcript,
        ) -> Option<(RangeProof, CompressedRistretto)> {
            commitment.generate_proof_of_knowledge(&self.pc_gens, &self.bp_gens, prover_transcript)
        }

    pub fn verify_proof_of_knowledge(
        &self,
        commitment: &PedersenCommitment,
        proof: &RangeProof,
        verifier_transcript: Transcript,
        committed_value: &CompressedRistretto,
    ) -> bool {
        commitment.verify_proof_of_knowledge(
            &self.pc_gens,
            &self.bp_gens,
            proof,
            verifier_transcript,
            committed_value,
        )
    }
}

impl VotingSystem {
    pub fn new(transcript: &'static [u8]) -> VotingSystem {
        VotingSystem {
            transcript
        }
    }

    pub fn cast_vote(
        &self, 
        candidate: CandidateId, 
        zkp_value: ZKPCommitmentValue, 
        zkp_randomness: &[u8],
    ) -> Vote {
        let zkm = ZKPModule::new();
        let hash = Sha512::digest(zkp_randomness);
        let mut hash_bytes = [0u8; 64];
        hash_bytes.copy_from_slice(hash.as_slice());
        let randomness = Scalar::from_bytes_mod_order_wide(&hash_bytes);

        let prover_transcript = Transcript::new(self.transcript);

        let commitment = ZKPModule::generate_commitment(zkp_value, randomness);
        let (proof, committed_value) = zkm.generate_proof_of_knowledge(
            &commitment,
            prover_transcript,
        ).expect("Failed to generate range proof");
        Vote {
            candidate,
            commitment,
            proof,
            committed_value,
        }
    }

    pub fn verify_vote(
        &self,
        vote: &Vote,
    ) -> bool {
        let zkm = ZKPModule::new();
        let verifier_transcript = Transcript::new(self.transcript);
        zkm.verify_proof_of_knowledge(
            &vote.commitment, 
            &vote.proof,
            verifier_transcript,
            &vote.committed_value,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_proof_of_knowledge_and_verify() {
        let value = 42;
        let transcript = b"ZKPExample";
        let zkp_randomness = b"hello world";
        let hash = Sha512::digest(zkp_randomness);
        let mut hash_bytes = [0u8; 64];
        hash_bytes.copy_from_slice(hash.as_slice());

        // Construct a scalar from the byte array
        let randomness = Scalar::from_bytes_mod_order_wide(&hash_bytes);

        let prover_transcript = Transcript::new(transcript);

        let zkm = ZKPModule::new();
        let commitment = ZKPModule::generate_commitment(value, randomness);
        let (proof, committed_value) = zkm.generate_proof_of_knowledge(
            &commitment,
            prover_transcript,
        ).expect("Failed to generate range proof");
        
        let verifier_transcript = Transcript::new(transcript);
        assert!(zkm.verify_proof_of_knowledge(
            &commitment, 
            &proof,
            verifier_transcript,
            &committed_value,
        ));
    }

    #[test]
    fn test_cast_vote_and_verify() {
        let candidate = 1;
        let value = 42;
        let transcript = b"ZKPExample";
        let zkp_randomness = b"hello world";

        let vs = VotingSystem::new(transcript);
        let vote = vs.cast_vote(
            candidate, 
            value, 
            zkp_randomness, 
        );
        
        assert!(vs.verify_vote(
            &vote,
        ));
    }
}

