const elliptic = require('elliptic');
const crypto = require('crypto');

// Define the elliptic curve and create an instance
const ec = new elliptic.ec('secp256k1');

// User A (Admin) key pair generation
const userAKeyPair = ec.genKeyPair();

// User B key pair generation
const userBKeyPair = ec.genKeyPair();

// Generate a commitment to the group (e.g., "Admin")
const groupCommitment = crypto.createHash('sha256').update('Admin').digest('hex');
const groupCommitment2 = crypto.createHash('sha256').update('Member').digest('hex');

// User A generates a proof of knowledge of group membership
const userAProof = generateMembershipProof(userAKeyPair, groupCommitment);

// User B tries to generate a proof, but should fail
const userBProof = generateMembershipProof(userBKeyPair, groupCommitment2);

console.log('User A proof:', userAProof);
console.log('User B proof:', userBProof);

// Verify the proofs
const isValidUserA = verifyMembershipProof(userAProof, groupCommitment, userAKeyPair.getPublic());
const isValidUserB = verifyMembershipProof(userBProof, groupCommitment, userBKeyPair.getPublic());

console.log('Is UserA member of group Admin?', isValidUserA);
console.log('Is UserB member of group Admin?', isValidUserB);

// Function to generate a proof of knowledge of group membership
function generateMembershipProof(keyPair, groupCommitment) {
    const commitment = keyPair.getPublic().encode('hex', true);
    const randomValue = crypto.randomBytes(32).toString('hex');

    const challenge = crypto.createHash('sha256')
        .update(groupCommitment)
        .update(commitment)
        .update(randomValue)
        .digest('hex');

    const response = ec.keyFromPrivate(randomValue, 'hex').getPrivate().toString(16);

    return { commitment, challenge, response };
}

// Function to verify a proof of knowledge of group membership
function verifyMembershipProof(proof, groupCommitment, publicKey) {
    const { commitment, challenge, response } = proof;

    const newChallenge = crypto.createHash('sha256')
        .update(groupCommitment)
        .update(commitment)
        .update(response)
        .digest('hex');

    return newChallenge === challenge && ec.keyFromPublic(publicKey, 'hex').getPublic().encode('hex', true) === commitment;
}