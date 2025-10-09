# compile
circom multiplier.circom --r1cs --wasm --sym -o build

# pot ceremony
snarkjs powersoftau new bn128 12 pot12_0000.ptau -v
snarkjs powersoftau contribute pot12_0000.ptau pot12_0001.ptau --name="first contrib" -v
snarkjs powersoftau prepare phase2 pot12_0001.ptau pot12_final.ptau -v

# setup & zkey
snarkjs groth16 setup build/multiplier.r1cs pot12_final.ptau build/multiplier_0000.zkey
snarkjs zkey contribute build/multiplier_0000.zkey build/multiplier_final.zkey --name="one-party" -v
snarkjs zkey export verificationkey build/multiplier_final.zkey build/verification_key.json

# witness
node build/multiplier_js/generate_witness.js build/multiplier_js/multiplier.wasm input.json build/witness.wtns

# prove
snarkjs groth16 prove build/multiplier_final.zkey build/witness.wtns build/proof.json build/public.json

# verify locally
snarkjs groth16 verify build/verification_key.json build/public.json build/proof.json

# export verifier
snarkjs zkey export solidityverifier build/multiplier_final.zkey build/verifier.sol

# get solidity calldata for a quick test
snarkjs groth16 calldata build/proof.json build/public.json
