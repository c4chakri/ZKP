// examples/inner.rs
use plonky2::field::goldilocks_field::GoldilocksField as F;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::prove::prove;
use plonky2::plonk::config::GenericConfig;
use plonky2::util::timing::TimingTree;

fn main() {
    // Choose a config (security / generic params)
    type C = PoseidonGoldilocksConfig;

    // 1) Make a circuit builder
    let mut builder = CircuitBuilder::<F, 2>::new(C::default());

    // 2) Add wires / inputs
    // We'll add a and b as private witnesses and c as public output.
    // Plonky2 commonly works with witness slots; add them and then constrain.
    let a = builder.add_virtual_target();
    let b = builder.add_virtual_target();

    // Add multiplication constraint: c = a * b
    // In plonky2 you usually add arithmetic gates; here we use the helper API:
    let product = builder.mul(a, b);

    // designate product as public input (expose to verifier)
    let _public = builder.register_public_input(product);

    // 3) Finalize circuit
    let data = builder.build();

    // 4) Provide witness values and create the witness vector
    // We'll use a=3, b=11 => c=33
    let mut pw = plonky2::plonk::witness::PartialWitness::new();
    pw.set_target(a, F::from_canonical_u64(3));
    pw.set_target(b, F::from_canonical_u64(11));

    // 5) Prove (produce a proof)
    let mut timing = TimingTree::new("prove", plonky2::util::timing::TimingTreeNode::new("root"));
    let proof: ProofWithPublicInputs<F, C> =
        prove(&data, &pw, &mut timing).expect("proving failed");

    // 6) Print proof public inputs (should include 33)
    println!("Public inputs: {:?}", proof.proof.public_inputs);

    // Save proof if you want (e.g., as bincode/serde JSON).
    // e.g.: fs::write("inner_proof.bin", bincode::serialize(&proof).unwrap()).unwrap();
}
