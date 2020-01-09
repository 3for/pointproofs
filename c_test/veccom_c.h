/* Text to put at the beginning of the generated file. Testing */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define COMMIT_LEN 97

#define PROOF_LEN 97

#define RAW_PP_LEN 196625

#define VP_LEN 49737

/**
 * deserialized commitment struct
 */
typedef struct vcp_commitment {
  void *data;
} vcp_commitment;

/**
 * deserialized prover parameter struct
 */
typedef struct vcp_pp {
  void *data;
} vcp_pp;

/**
 * values
 */
typedef struct vcp_value {
  const uint8_t *data;
  size_t len;
} vcp_value;

/**
 * serialized commitment struct
 */
typedef struct vcp_commitment_bytes {
  uint8_t data[COMMIT_LEN];
} vcp_commitment_bytes;

/**
 * deserialized proof struct
 */
typedef struct vcp_proof {
  void *data;
} vcp_proof;

/**
 * deserialized verifier parameter struct
 */
typedef struct vcp_vp {
  void *data;
} vcp_vp;

/**
 * non-serialized
 */
typedef struct vcp_params {
  vcp_pp prover;
  vcp_vp verifier;
} vcp_params;

/**
 * serialized prover parameter struct
 */
typedef struct vcp_pp_bytes {
  uint8_t data[RAW_PP_LEN];
} vcp_pp_bytes;

/**
 * serialized proof struct
 */
typedef struct vcp_proof_bytes {
  uint8_t data[PROOF_LEN];
} vcp_proof_bytes;

/**
 * serialized verifer parameter struct
 */
typedef struct vcp_vp_bytes {
  uint8_t data[VP_LEN];
} vcp_vp_bytes;

/**
 * Generate a commitment
 */
vcp_commitment vcp_commit(vcp_pp prover, const vcp_value *values, uintptr_t n);

/**
 * Deserializeing bytes into commitments
 */
vcp_commitment vcp_commit_deserial(vcp_commitment_bytes commit);

/**
 * Serializing commitments into bytes
 */
vcp_commitment_bytes vcp_commit_serial(vcp_commitment commit);

/**
 * update an existing commitment
 */
vcp_commitment vcp_commit_update(vcp_pp prover,
                                 vcp_commitment com,
                                 size_t changed_idx,
                                 vcp_value val_old,
                                 vcp_value val_new);

/**
 * Free commitment
 */
void vcp_free_commit(vcp_commitment commit);

/**
 * Free proof
 */
void vcp_free_proof(vcp_proof proof);

/**
 * Free prover parameter
 */
void vcp_free_prover_params(vcp_pp pp);

/**
 * Free verifier parameter
 */
void vcp_free_verifier_params(vcp_vp vp);

/**
 * Generating a pair of parameters
 */
vcp_params vcp_paramgen(const uint8_t *seedbuf, size_t seedlen, uint8_t ciphersuite, size_t n);

/**
 * Deserializing bytes into prover parameters
 */
vcp_pp vcp_pp_deserial(vcp_pp_bytes pprover);

/**
 * Serializing a prove parameter into bytes
 */
vcp_pp_bytes vcp_pp_serial(vcp_pp pprover);

/**
 * Deserializeing bytes into proofs
 */
vcp_proof vcp_proof_deserial(vcp_proof_bytes proof);

/**
 * Serializing proofs into bytes
 */
vcp_proof_bytes vcp_proof_serial(vcp_proof proof);

/**
 * update an existing proof
 */
vcp_proof vcp_proof_update(vcp_pp prover,
                           vcp_proof proof,
                           size_t idx,
                           size_t changed_idx,
                           vcp_value val_old,
                           vcp_value val_new);

/**
 * Generate a proof
 */
vcp_proof vcp_prove(vcp_pp prover, const vcp_value *values, uintptr_t n, size_t idx);

/**
 * aggregate proofs within a same commitment
 */
vcp_proof vcp_same_commit_aggregate(vcp_commitment com,
                                    const vcp_proof *proofs,
                                    const size_t *set,
                                    const vcp_value *values,
                                    size_t nvalues,
                                    size_t param_n);

/**
 * verify an aggregated proof within a same commitment
 */
bool vcp_same_commit_batch_verify(vcp_vp verifier,
                                  vcp_commitment com,
                                  vcp_proof proof,
                                  const size_t *set,
                                  const vcp_value *values,
                                  size_t nvalues);

/**
 * verify the proof against the value and commitment
 */
bool vcp_verify(vcp_vp verifier, vcp_commitment com, vcp_proof proof, vcp_value value, size_t idx);

/**
 * Deserializing bytes into verifier parameters
 */
vcp_vp vcp_vp_deserial(vcp_vp_bytes pverifier);

/**
 * Serializing a verifier parameter into bytes
 */
vcp_vp_bytes vcp_vp_serial(vcp_vp pverifier);

/**
 * aggregated proofs cross commitments
 */
vcp_proof vcp_x_commit_aggregate_full(const vcp_commitment *com,
                                      const vcp_proof *proof,
                                      const size_t *set,
                                      const vcp_value *values,
                                      const size_t *commit_indices,
                                      size_t no_commits,
                                      size_t param_n);

/**
 * aggregated proofs cross commitments
 */
vcp_proof vcp_x_commit_aggregate_partial(const vcp_commitment *com,
                                         const vcp_proof *agg_proof,
                                         const size_t *set,
                                         const vcp_value *values,
                                         const size_t *commit_indices,
                                         size_t no_commits,
                                         size_t param_n);

/**
 * verify an aggregated proof across commitments
 */
bool vcp_x_commit_batch_verify(vcp_vp verifier,
                               const vcp_commitment *com,
                               vcp_proof proof,
                               const size_t *set,
                               const vcp_value *values,
                               const size_t *commit_indices,
                               size_t no_commits);