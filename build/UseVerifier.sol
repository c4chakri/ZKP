// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./verifier.sol";

contract UseVerifier is Groth16Verifier {
    // Pass the raw proof / public signals as returned by snarkjs
    function checkProof(
        uint[2] calldata a,
        uint[2][2] calldata b,
        uint[2] calldata c,
        uint[3] calldata input
    ) public view returns (bool) {
        return verifyProof(a, b, c, input);
    }
}
