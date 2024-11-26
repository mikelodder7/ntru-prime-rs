/// Length of public key in bytes for sntrup761
pub const PUBLIC_KEY_LENGTH: usize = 1158;
/// Length of secret key in bytes for sntrup761
pub const SECRET_KEY_LENGTH: usize = 1763;
/// Length of ciphertext in bytes for sntrup761
pub const CIPHERTEXT_LENGTH: usize = 1039;
/// Length of shared secret in bytes for sntrup761
pub const SHARED_SECRET_LENGTH: usize = 32;
/// Name of the algorithm as defined in the NTRU Prime specification
pub const ALGORITHM_NAME: &'static str = "sntrup761";
/// NIST security level claimed by sntrup761 (2 = AES-128 equivalent)
pub const CLAIMED_NIST_LEVEL: usize = 2;
/// Size parameter used for padding in sorting operations
pub const PPADSORT: usize = 761;

/// Floor of 2^31/q, used for Barrett reduction
pub const Q31: usize = 467759;
/// Closest integer to 2^27/q, used for Barrett reduction
pub const Q27: usize = 29235;
/// Closest integer to 2^18/q, used for Barrett reduction
pub const Q18: usize = 57;
/// Closest integer to 2^14/q, used for Barrett reduction
pub const Q14: usize = 4;
/// Degree of the ring polynomial, p=761 for sntrup761
pub const P: usize = 761;
/// Modulus for the ring R_q = Z_q[x]/(x^p-x-1), q=4591 for sntrup761
pub const Q: usize = 4591;
/// Weight of secret key polynomials, w=286 for sntrup761
pub const W: usize = 286;
