// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IProofHub {
    function verifyDirect(uint[2] memory a, uint[2][2] memory b, uint[2] memory c, uint[] memory input) external view returns (bool);
}

contract Consumer {
    IProofHub public hub;

    constructor(address _hub) { hub = IProofHub(_hub); }

    function doActionWithProof(
        uint[2] memory a,
        uint[2][2] memory b,
        uint[2] memory c,
        uint[] memory input
    ) external {
        require(hub.verifyDirect(a,b,c,input), "Proof invalid");
        // do action (e.g., mint, transfer, change state)
    }
}
