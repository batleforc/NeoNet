use secp256k1_zkp::{generate_keypair, All, Keypair, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[allow(dead_code)]
#[derive(Debug)]
struct GroupMembershipProof {
    pub commitment: String,
    pub challenge: String,
    pub response: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Zkp {
    pub server_commitment: String,
    mystery: String,
    secp: Secp256k1<All>,
}

impl Zkp {
    #[allow(dead_code)]
    pub fn new(mystery: String) -> Self {
        Zkp {
            server_commitment: Zkp::sha256(mystery.as_bytes()),
            mystery,
            secp: Secp256k1::new(),
        }
    }
    #[allow(dead_code)]
    pub fn gen_key_pair() -> (SecretKey, PublicKey) {
        generate_keypair(&mut rand::thread_rng())
    }
    #[allow(dead_code)]
    pub fn sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher
            .finalize()
            .iter()
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:02x}");
                output
            })
    }
    #[allow(dead_code)]
    pub fn generate_membership_proof(
        &self,
        public_key: &PublicKey,
        group_commitment: String,
    ) -> GroupMembershipProof {
        let commitment = public_key.to_string();
        let random_value: String =
            (0..32)
                .map(|_| rand::random::<u8>())
                .fold(String::new(), |mut output, b| {
                    let _ = write!(output, "{b:02x}");
                    output
                });

        let mut challenge_data = Vec::new();
        challenge_data.extend_from_slice(group_commitment.as_bytes());
        challenge_data.extend_from_slice(commitment.as_bytes());
        challenge_data.extend_from_slice(random_value.as_bytes());

        let challenge = Zkp::sha256(&challenge_data);
        let response = match Keypair::from_seckey_str(&self.secp, &random_value) {
            Ok(keypair) => keypair.secret_key().display_secret(),
            Err(_) => todo!(),
        };
        GroupMembershipProof {
            commitment,
            challenge,
            response: response.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn verify_membership_proof(
        &self,
        proof: GroupMembershipProof,
        group_commitment: String,
        public_key: &PublicKey,
    ) -> bool {
        let mut data = Vec::new();
        data.extend_from_slice(group_commitment.as_bytes());
        data.extend_from_slice(proof.commitment.as_bytes());
        data.extend_from_slice(proof.response.as_bytes());

        let new_challenge = Zkp::sha256(&data);

        if proof.challenge == new_challenge {
            return public_key.to_string() == proof.commitment;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zkp() {
        let mystery_key = String::from("JosephJoestar");
        let zkp = Zkp::new(mystery_key);
        let group_commitment = Zkp::sha256(b"Admin");
        let group_commitment2 = Zkp::sha256(b"Member");

        let (_secret_key_user_a, public_key_user_a) = Zkp::gen_key_pair();
        let (_secret_key_user_b, public_key_user_b) = Zkp::gen_key_pair();

        let user_a_proof =
            zkp.generate_membership_proof(&public_key_user_a, group_commitment.clone());
        let user_b_proof =
            zkp.generate_membership_proof(&public_key_user_b, group_commitment2.clone());

        let is_valid_user_a =
            zkp.verify_membership_proof(user_a_proof, group_commitment.clone(), &public_key_user_a);
        let is_valid_user_b =
            zkp.verify_membership_proof(user_b_proof, group_commitment, &public_key_user_b);

        assert!(is_valid_user_a);
        assert!(!is_valid_user_b);

        assert!(true);
    }
}
