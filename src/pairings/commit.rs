//! This file is part of the veccom crate.
//! It defines APIs for constructing and updating commitments.

use ff::{Field, PrimeField};
use pairing::{bls12_381::*, CurveAffine, CurveProjective};
use pairings::err::*;
use pairings::hash_to_field_veccom::*;
use pairings::param::*;
use pairings::*;

impl Commitment {
    /// generate a new commitment.
    ///     * input: prover parameter set
    ///     * input: a list of n values
    ///     * output: a commitment
    ///     * error: invalid ciphersuite/parameters
    pub fn new<Blob: AsRef<[u8]>>(
        prover_params: &ProverParams,
        values: &[Blob],
    ) -> Result<Self, String> {
        // checks that cipersuite is supported
        if !check_ciphersuite(prover_params.ciphersuite) {
            return Err(ERR_CIPHERSUITE.to_owned());
        };

        if prover_params.n != values.len() {
            return Err(ERR_INVALID_VALUE.to_owned());
        };

        // hash the values into scalars
        let scalars_fr_repr: Vec<FrRepr> = values
            .iter()
            .map(|s| hash_to_field_repr_veccom(s.as_ref()))
            .collect();
        let scalars_u64: Vec<&[u64; 4]> = scalars_fr_repr.iter().map(|s| &s.0).collect();

        // commit = \prod pp[i]^scalar[i]
        let commit = {
            if prover_params.precomp.len() == 512 * prover_params.n {
                VeccomG1Affine::sum_of_products_precomp_256(
                    &prover_params.generators[0..prover_params.n],
                    &scalars_u64,
                    &prover_params.precomp,
                )
            } else {
                VeccomG1Affine::sum_of_products(
                    &prover_params.generators[0..prover_params.n],
                    &scalars_u64,
                )
            }
        };

        Ok(Self {
            ciphersuite: prover_params.ciphersuite,
            commit,
        })
    }

    /// upated an existing commitment
    ///     * input: commitment
    ///     * input: prover parameter set
    ///     * input: the index of the value to be updated
    ///     * input: the old value
    ///     * input: the new value
    ///     * output: mutate self to the new commitment
    ///     * error: invalid ciphersuite, parameters
    pub fn update<Blob: AsRef<[u8]>>(
        &mut self,
        prover_params: &ProverParams,
        changed_index: usize,
        value_before: Blob,
        value_after: Blob,
    ) -> Result<(), String> {
        // checks that cipersuite is supported
        if self.ciphersuite != prover_params.ciphersuite {
            return Err(ERR_CIPHERSUITE.to_owned());
        }
        if !check_ciphersuite(prover_params.ciphersuite) {
            return Err(ERR_CIPHERSUITE.to_owned());
        }
        if prover_params.n <= changed_index {
            return Err(ERR_INVALID_INDEX.to_owned());
        };

        // multiplier = hash(new_value) - hash(old_value)
        let mut multiplier = hash_to_field_veccom(&value_before);
        multiplier.negate();
        multiplier.add_assign(&hash_to_field_veccom(&value_after));

        // new_commit = old_commit * g[index]^multiplier
        let res = if prover_params.precomp.len() == 3 * prover_params.generators.len() {
            prover_params.generators[changed_index].mul_precomp_3(
                multiplier,
                &prover_params.precomp[changed_index * 3..(changed_index + 1) * 3],
            )
        } else if prover_params.precomp.len() == 256 * prover_params.generators.len() {
            prover_params.generators[changed_index].mul_precomp_256(
                multiplier,
                &prover_params.precomp[changed_index * 256..(changed_index + 1) * 256],
            )
        } else {
            prover_params.generators[changed_index].mul(multiplier)
        };

        self.commit.add_assign(&res);
        Ok(())
    }

    /// upated an existing commitment with a list of messages
    ///     * input: commitment
    ///     * input: prover parameter set
    ///     * input: the indices of the value to be updated
    ///     * input: the old values
    ///     * input: the new values
    ///     * output: mutate self to the new commitment
    ///     * error: invalid ciphersuite, parameters
    pub fn batch_update<Blob: AsRef<[u8]>>(
        &mut self,
        prover_params: &ProverParams,
        changed_index: &[usize],
        value_before: &[Blob],
        value_after: &[Blob],
    ) -> Result<(), String> {
        // checks that cipersuite is supported
        if self.ciphersuite != prover_params.ciphersuite {
            return Err(ERR_CIPHERSUITE.to_owned());
        }
        if !check_ciphersuite(prover_params.ciphersuite) {
            return Err(ERR_CIPHERSUITE.to_owned());
        }
        // check the parameters are valid
        for index in changed_index {
            if prover_params.n <= *index {
                return Err(ERR_INVALID_INDEX.to_owned());
            };
        }
        if changed_index.len() != value_before.len() || changed_index.len() != value_after.len() {
            return Err(ERR_INDEX_VALUE_NOT_MATCH.to_owned());
        }
        if !misc::has_unique_elements(changed_index) {
            return Err(ERR_DUPLICATED_INDEX.to_owned());
        }

        // get the scalars from the hashes
        let mut multiplier_set: Vec<FrRepr> = vec![];
        for i in 0..value_before.len() {
            // multiplier = hash(new_value) - hash(old_value)
            let mut multiplier = hash_to_field_veccom(&value_before[i]);
            multiplier.negate();
            multiplier.add_assign(&hash_to_field_veccom(&value_after[i]));
            multiplier_set.push(multiplier.into_repr());
        }
        let scalars_u64: Vec<&[u64; 4]> = multiplier_set.iter().map(|s| &s.0).collect();

        // form the basis for `sum_of_products`
        let mut basis: Vec<VeccomG1Affine> = vec![];
        for e in changed_index.iter().take(value_before.len()) {
            basis.push(prover_params.generators[*e]);
        }
        let mut pre: Vec<VeccomG1Affine> = vec![];
        // compute delta = \prod g[index]^multiplier
        let delta = {
            // to use sum_of_products with pre_computation,
            // we need to form the right basis
            if prover_params.precomp.len() == 256 * prover_params.generators.len() {
                for e in changed_index.iter().take(value_before.len()) {
                    pre = [
                        pre,
                        prover_params.precomp
                            [changed_index[*e] * 256..(changed_index[*e] + 1) * 256]
                            .to_vec(),
                    ]
                    .concat();
                }
                VeccomG1Affine::sum_of_products_precomp_256(&basis, &scalars_u64, &pre)
            } else {
                // without pre_computation
                VeccomG1Affine::sum_of_products(&basis[..], &scalars_u64)
            }
        };
        // new_commit = old_commit * \prod g[index]^multiplier
        self.commit.add_assign(&delta);
        Ok(())
    }
}
