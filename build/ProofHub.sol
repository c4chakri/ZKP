// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./verifier.sol";

contract ProofHub {
    Groth16Verifier public verifier;
    // simple replay protection: store hash(proof, publicInputs) -> bool
    mapping(bytes32 => bool) public seen;

    event ProofVerified(address indexed prover, bytes32 indexed proofHash, uint[3] publicSignals);

    constructor(address _verifier) {
        verifier = Groth16Verifier(_verifier);
    }

    // Verify the proof and emit an event or store result for other contracts
    function verifyAndRegister(
        uint[2] memory a,
        uint[2][2] memory b,
        uint[2] memory c,
        uint[3] memory publicInputs
    ) external returns (bool) {
        // Bind proof to context to avoid replay across contracts / chains
        // e.g., include address(this) or chainId in public inputs before proving.
        bytes32 h = keccak256(abi.encodePacked(a, b, c, publicInputs));
        require(!seen[h], "Proof already used");
        bool ok = verifier.verifyProof(a, b, c, publicInputs);
        require(ok, "Invalid proof");
        seen[h] = true;
        emit ProofVerified(msg.sender, h, publicInputs);
        return true;
    }

    // helper: allow other contracts to call verifier directly (if they trust this hub)
    function verifyDirect(
        uint[2] memory a,
        uint[2][2] memory b,
        uint[2] memory c,
        uint[3] memory publicInputs
    ) external view returns (bool) {
        return verifier.verifyProof(a, b, c, publicInputs);
    }
}
