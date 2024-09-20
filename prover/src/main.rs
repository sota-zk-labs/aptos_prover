use stark_evm_adapter::annotated_proof::AnnotatedProof;
use stark_evm_adapter::annotation_parser::{split_fri_merkle_statements, SplitProofs};

fn main() {
    let proof_file = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/bootloader_serialized_proof.json"
    ));
    let annotated_proof: AnnotatedProof = serde_json::from_str(proof_file).unwrap();

    // create the split proof
    let split_proofs: SplitProofs = split_fri_merkle_statements(annotated_proof).unwrap();
    println!("{:?}", split_proofs.main_proof);
}
