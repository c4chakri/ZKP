// examples/aggregate.rs
use plonky2::field::goldilocks_field::GoldilocksField as F;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::proof::ProofWithPublicInputs;
use plonky2::plonk::prove::prove;
use plonky2::plonk::verifier::verify;
use plonky2::plonk::config::GenericConfig;
use plonky2::util::timing::TimingTree;
use plonky2::recursion::verifier::prepare_verifier_data_for_recursive_proof;
use plonky2::recursion::circuit::verify_proof_in_circuit;

fn main() {
    type C = PoseidonGoldilocksConfig;

    // --- STEP A: produce an inner proof (same as examples/inner.rs) ---
    // For clarity we produce it inline; in practice you'd load it from disk or another process.
    let mut builder_inner = CircuitBuilder::<F, 2>::new();
    let a = builder_inner.add_virtual_target();
    let b = builder_inner.add_virtual_target();
    let product = builder_inner.mul(a, b);
    let _pub = builder_inner.register_public_input(product);
    let data_inner = builder_inner.build();

    let mut pw_inner = plonky2::plonk::witness::PartialWitness::new();
    pw_inner.set_target(a, F::from_canonical_u64(3));
    pw_inner.set_target(b, F::from_canonical_u64(11));

    let mut timing = TimingTree::new("prove-inner", plonky2::util::timing::TimingTreeNode::new("root"));
    let proof_inner: ProofWithPublicInputs<F, C> =
        prove(&data_inner, &pw_inner, &mut timing).expect("inner prove failed");

    println!("Inner proof public inputs: {:?}", proof_inner.proof.public_inputs);

    // --- STEP B: Prepare verifier data so we can call the verifier gadget inside a circuit ---
    // Plonky2 provides helper functions to prepare the verifier data for recursive usage.
    let verifier_data = prepare_verifier_data_for_recursive_proof(&data_inner)
        .expect("prepare verifier data failed");

    // --- STEP C: Build outer circuit that verifies the inner proof in-circuit ---
    let mut builder_outer = CircuitBuilder::<F, 2>::new();

    // For recursive verification we use a helper gadget that takes the inner proof bytes + public inputs
    // and enforces verification. The exact API on `verify_proof_in_circuit` might vary by version.
    // We'll create virtual targets for the serialized inner proof and public inputs to pass to the gadget.
    // (In practice you might pack the inner proof bytes into field elements and feed them as targets.)
    let _serialized_proof_target = builder_outer.add_virtual_target(); // placeholder
    let _public_inputs_target = builder_outer.add_virtual_target(); // placeholder

    // This is a conceptual call — actual signature may require arrays of targets or specific layouts
    verify_proof_in_circuit(
        &mut builder_outer,
        &verifier_data,
        /* proof target(s) */ &_serialized_proof_target,
        /* public inputs target(s) */ &_public_inputs_target,
    ).expect("gadget failed");

    // register no extra public inputs for outer (the outer proof simply attests inner proof valid)
    let data_outer = builder_outer.build();

    // Create a witness for the outer circuit by serializing inner proof into inner field elements
    let mut pw_outer = plonky2::plonk::witness::PartialWitness::new();
    // ... serialization & assignment steps here ...
    // The exact serialization layout depends on plonky2 version; see "prepare_verifier_data_for_recursive_proof" docs.

    // Prove outer (this produces the recursive/aggregated proof)
    let mut timing_outer = TimingTree::new("prove-outer", plonky2::util::timing::TimingTreeNode::new("root"));
    let proof_outer: ProofWithPublicInputs<F, C> =
        prove(&data_outer, &pw_outer, &mut timing_outer).expect("outer prove failed");

    println!("Outer proof generated. Public inputs: {:?}", proof_outer.proof.public_inputs);

    // Optionally verify outer proof locally using normal verify (this verifies that outer is valid)
    let verified = verify(&proof_outer.proof, &data_outer.verifier_data).expect("verify call failed");
    println!("Outer proof verification result: {}", verified);
}
