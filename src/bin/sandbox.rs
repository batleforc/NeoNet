extern crate neonet;
use p256::ecdsa::signature::Verifier;
use p256::ecdsa::Signature;
use p256::ecdsa::{signature::Signer, SigningKey, VerifyingKey};
use p256::EncodedPoint;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct GroupMembershipProof {
    commitment: Vec<u8>,
    challenge: Vec<u8>,
    response: Vec<u8>,
}

fn main() {
    // User A (Admin) key pair generation
    let user_a_key_pair = SigningKey::random(&mut OsRng);

    // User B key pair generation
    let user_b_key_pair = SigningKey::random(&mut OsRng);

    // Generate a commitment to the group (e.g., "Admin")
    let group_commitment = sha256(b"Admin");
    let group_commitment2 = sha256(b"Member");

    println!(
        "Group commitment Admin !== Member: {:?}",
        group_commitment != group_commitment2
    );

    // User A generates a proof of knowledge of group membership
    let user_a_proof = generate_membership_proof(&user_a_key_pair, &group_commitment);

    // User B tries to generate a proof, but should fail
    let user_b_proof = generate_membership_proof(&user_b_key_pair, &group_commitment2);

    println!("User A proof: {:?}", user_a_proof);
    println!("User B proof: {:?}", user_b_proof);

    // Verify the proofs
    let is_valid_user_a = verify_membership_proof(
        user_a_proof,
        &group_commitment,
        user_a_key_pair
            .verifying_key()
            .to_encoded_point(false)
            .to_bytes()
            .to_vec(),
    );
    let is_valid_user_b = verify_membership_proof(
        user_b_proof,
        &group_commitment,
        user_b_key_pair
            .verifying_key()
            .to_encoded_point(false)
            .to_bytes()
            .to_vec(),
    );

    println!("Is UserA member of group Admin? {}", is_valid_user_a);
    println!("Is UserB member of group Admin? {}", is_valid_user_b);
}

// Function to generate a proof of knowledge of group membership
fn generate_membership_proof(
    key_pair: &SigningKey,
    group_commitment: &[u8],
) -> GroupMembershipProof {
    let public_key = key_pair.verifying_key();

    let random_value: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();

    let mut data = Vec::new();
    data.extend_from_slice(group_commitment);
    data.extend_from_slice(&public_key.to_encoded_point(true).to_bytes());
    data.extend_from_slice(&random_value);

    let challenge = sha256(&data);

    let signature: Signature = key_pair.sign(&random_value);
    GroupMembershipProof {
        commitment: public_key.to_encoded_point(true).as_bytes().to_vec(),
        challenge,
        response: signature.to_bytes().to_vec(),
    }
}

fn verify_membership_proof(
    proof: GroupMembershipProof,
    group_commitment: &[u8],
    public_key_bytes: Vec<u8>,
) -> bool {
    //let (commitment, challenge, response) = proof;

    let mut data = Vec::new();
    data.extend_from_slice(group_commitment);
    data.extend_from_slice(&proof.commitment);
    data.extend_from_slice(&proof.response);

    let new_challenge = sha256(&data);

    println!("Challenge equal ? {}", *proof.challenge == new_challenge);

    match EncodedPoint::from_bytes(public_key_bytes)
        .map(|encoded_point| VerifyingKey::from_encoded_point(&encoded_point))
    {
        Ok(public_key_res) => {
            let public_key = public_key_res.unwrap();
            if let Ok(signature) = p256::ecdsa::Signature::from_der(&proof.response) {
                if public_key.verify(&new_challenge, &signature).is_ok() {
                    return true;
                }
            }
        }
        Err(_) => {
            panic!("Failed to decode public key");
        }
    }

    false
}

// Helper function for SHA256 hashing
fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
