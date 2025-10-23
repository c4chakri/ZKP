pragma circom 2.1.6;

include "circomlib/poseidon.circom"; // circomlib poseidon

template Preimage() {
    // private input (the secret)
    signal input x;

    // public input (the hash)
    signal output h;

    component hasher = Poseidon(1);

    hasher.inputs[0] <== x;
    h <== hasher.out;
}

component main = Preimage();