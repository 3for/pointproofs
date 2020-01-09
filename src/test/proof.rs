use pairings::param::paramgen_from_seed;
use pairings::*;

#[test]
fn negative_test_proof() {
    let n = 8usize;
    let (prover_params, verifier_params) =
        paramgen_from_seed("This is Leo's Favourite very very very long Seed", 0, n).unwrap();
    let mut prover_params2 = prover_params.clone();
    prover_params2.ciphersuite = 1;
    let mut verifier_params2 = verifier_params.clone();
    verifier_params2.ciphersuite = 1;

    let mut prover_params256 = prover_params.clone();
    prover_params256.precomp_256();

    let mut init_values = Vec::with_capacity(n);
    for i in 0..n {
        let s = format!("this is message number {}", i);
        init_values.push(s.into_bytes());
    }

    let mut values: Vec<&[u8]> = Vec::with_capacity(n);
    for e in init_values.iter().take(n) {
        values.push(&e);
    }
    let mut values2 = Vec::with_capacity(n + 1);
    for i in 0..=n {
        values2.push(format!("new string {}", i).into_bytes());
    }

    assert!(Proof::new(&prover_params, &values, 9).is_err());
    assert!(Proof::new(&prover_params2, &values, 0).is_err());
    assert!(Proof::new(&prover_params, &values2, 0).is_err());

    let mut proof = Proof::new(&prover_params, &values, 0).unwrap();

    assert!(proof
        .update(&prover_params2, 0, 1, &values2[1], &values2[2])
        .is_err());
    assert!(proof
        .update(&prover_params2, 0, 9, &values2[1], &values2[2])
        .is_err());

    let mut proof2 = proof.clone();
    proof2.ciphersuite = 1;
    assert!(proof2
        .update(&prover_params, 0, 1, &values2[1], &values2[2])
        .is_err());

    let com = Commitment::new(&prover_params, &values).unwrap();
    let mut com2 = com.clone();
    com2.ciphersuite = 1;
    assert!(!proof.verify(&verifier_params2, &com, &values[0], 0));
    assert!(!proof.verify(&verifier_params, &com, &values[0], 9));
    assert!(!proof.verify(&verifier_params, &com2, &values[0], 0));
}