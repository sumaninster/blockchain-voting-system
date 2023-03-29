   
use rand::{RngCore, SeedableRng, thread_rng};
use pairing::{PairingCurveAffine, Engine};
use rand_chacha::ChaChaRng;
use bls12_381::Bls12;

// Define the curve and field types
type E = Bls12;
type Fr = <E as Engine>::Fr;
type G1 = <E as Engine>::G1;

    pub fn generate_proof_of_knowledge(
    value: u64,
    randomness: Scalar,
    ) -> Option<RangeProof> {
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64, 1);

        let mut prover_transcript = Transcript::new(b"ZKPExample");
        let mut prover_rng = thread_rng();

        /*let (com_value, com_randomness) =
            pc_gens.commit(Scalar::from(value), randomness);*/
        let com_randomness =
            pc_gens.commit(Scalar::from(value), randomness);

        let proof = RangeProof::prove_single(
            &bp_gens,
            &pc_gens,
            &mut prover_transcript,
            value,
            &randomness,
            64,
        )
        .expect("Failed to generate range proof");

        if !com_value.verify(&pc_gens, &proof, &com_randomness) {
            None
        } else {
            Some(proof)
        }
    }

    proof.verify_single(&pc_gens, &gens, &self.to_affine()).is_ok()



    pub fn to_affine(&self) -> G1 {
        let mut bytes = vec![0u8; 64];
        bytes[0..32].copy_from_slice(&self.value.to_bytes());
        bytes[32..64].copy_from_slice(&self.randomness.to_bytes());
        G1::hash_bytes(&bytes)
    }





//Implement the ZKP Module:
pub struct ZKPModule<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ZKPModule<T> {
    pub fn generate_commitment(value: Fr, randomness: Fr) -> PedersenCommitment {
        PedersenCommitment::new(value, randomness)
    }

    pub fn generate_proof_of_knowledge(
        commitment: &PedersenCommitment,
        value: Fr,
        randomness: Fr,
        ) -> Option<RangeProof> {
            commitment.generate_proof_of_knowledge(
                value,
                randomness,
            )
        }

    pub fn verify_proof_of_knowledge(
        commitment: &PedersenCommitment,
        proof: &RangeProof,
    ) -> bool {
        commitment.verify_proof_of_knowledge(
            &PedersenGens::default(),
            &BulletproofGens::new(64, 1),
            proof,
        )
    }
}


// Implement the Voting System:
pub struct Vote {
    pub candidate: u32,
    pub commitment: PedersenCommitment,
    pub proof: RangeProof,
}

pub struct VotingSystem<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> VotingSystem<T> {
    pub fn cast_vote(candidate: u32, value: Fr, randomness: Fr) -> Vote {
        let commitment = ZKPModule::<T>::generate_commitment(value, randomness);
        let proof = ZKPModule::<T>::generate_proof_of_knowledge(
            &commitment,
            value,
            randomness,
        ).expect("Failed to generate range proof");
        Vote {
            candidate,
            commitment,
            proof,
        }
    }

    pub fn verify_vote(vote: &Vote) -> bool {
        ZKPModule::<T>::verify_proof_of_knowledge(&vote.commitment, &vote.proof)
            //&& vote.candidate < T::MaxCandidates::get()
    }
}



    /*pub fn from_bytes(bytes: &[u8]) -> Option<PedersenCommitment> {
        let mut value_bytes = [0u8; 32];
        let mut randomness_bytes = [0u8; 32];
        value_bytes.copy_from_slice(&bytes[0..32]);
        randomness_bytes.copy_from_slice(&bytes[32..64]);
        Some(PedersenCommitment {
            value: Fr::from_bytes(&value_bytes).unwrap(),
            randomness: Fr::from_bytes(&randomness_bytes).unwrap(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; 64];
        bytes[0..32].copy_from_slice(&self.value.to_bytes());
        bytes[32..64].copy_from_slice(&self.randomness.to_bytes());
        bytes
    }*/
        //&& vote.candidate < T::MaxCandidates::get()
        let pc_gens = PedersenGens::default();
        let bp_gens = BulletproofGens::new(64, 1);
            _phantom: std::marker::PhantomData<T>,




        //println!("Scalar: {:?}", scalar);
        // Generate a random message to hash
        /*let message = "hello world";

        // Hash the message using SHA-3 512
        let mut hasher = Sha3_512::new();
        hasher.update(message);
        let hash = hasher.finalize();

        // Convert the hash bytes to a Scalar
        let scalar = Scalar::hash_from_bytes::<Sha3_512>(&hash);

        println!("Scalar: {:?}", scalar);*/

            use sha3::{Digest, Sha3_512};

pairing = "0.23.0"
rand = "0.8.5"
rand_chacha = "0.3.1"
bls12_381 = "0.8.0"
sha3 = "0.10.6"

        //let randomness = Scalar::random(&mut thread_rng());
    use rand::thread_rng;




        let hash = Sha512::digest(message);
        let bytes = [
            0x9c, 0x5c, 0x5a, 0x89, 0x1f, 0xd8, 0x2b, 0x34, 
            0x10, 0x60, 0x0d, 0x9c, 0xd7, 0x30, 0xfa, 0x09, 
            0x2d, 0xe9, 0x70, 0x3d, 0x3d, 0x47, 0x43, 0x9f, 
            0x54, 0x04, 0x2b, 0xa2, 0x92, 0xca, 0x1e, 0xa8,
            0x9c, 0x5c, 0x5a, 0x89, 0x1f, 0xd8, 0x2b, 0x34, 
            0x10, 0x60, 0x0d, 0x9c, 0xd7, 0x30, 0xfa, 0x09, 
            0x2d, 0xe9, 0x70, 0x3d, 0x3d, 0x47, 0x43, 0x9f, 
            0x54, 0x04, 0x2b, 0xa2, 0x92, 0xca, 0x1e, 0xa8,
        ];

        
        let hash = Sha512::digest(message);
        let mut hash_bytes = [0u8; 64];
        hash_bytes.copy_from_slice(hash.as_slice());

        // Construct a scalar from the byte array
        let randomness = Scalar::from_bytes_mod_order_wide(&hash_bytes);
