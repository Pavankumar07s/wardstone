/// Represents an elliptic curve cryptography primitive used for digital
/// signatures and key establishment where f is the key size.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Ecc {
  pub f: u16,
}

/// Represents the Weierstrass curve P-224 over a prime field. Also
/// known as secp224r1.
#[no_mangle]
pub static P224: Ecc = Ecc { f: 224 };

/// Represents the Weierstrass curve P-256 over a prime field. Also
/// known as secp256r1.
#[no_mangle]
pub static P256: Ecc = Ecc { f: 256 };

/// Represents the Weierstrass curve P-384 over a prime field. Also
/// known as secp384r1.
#[no_mangle]
pub static P384: Ecc = Ecc { f: 384 };

/// Represents the Weierstrass curve P-521 over a prime field. Also
/// known as secp521r1.
#[no_mangle]
pub static P521: Ecc = Ecc { f: 521 };

/// Represents the Weierstrass curve W-25519 over a prime field.
#[no_mangle]
pub static W25519: Ecc = Ecc { f: 255 };

/// Represents the Weierstrass curve W-488 over a prime field.
#[no_mangle]
pub static W448: Ecc = Ecc { f: 448 };

/// Represents the Montgomery curve Curve25519 over a prime field.
#[no_mangle]
pub static Curve25519: Ecc = Ecc { f: 255 };

/// Represents the Montgomery curve Curve488 over a prime field.
#[no_mangle]
pub static Curve448: Ecc = Ecc { f: 448 };

/// Represents the twisted Edwards curve Edwards25519 over a prime
/// field.
#[no_mangle]
pub static Edwards25519: Ecc = Ecc { f: 255 };

/// Represents the twisted Edwards curve Edwards488 over a prime field.
#[no_mangle]
pub static Edwards448: Ecc = Ecc { f: 448 };

/// Represents the Edwards curve E448 over a prime field.
#[no_mangle]
pub static E448: Ecc = Ecc { f: 448 };

/// Represents the curve brainpoolP224r1 specified in [RFC 5639].
///
/// [RFC 5639]: https://datatracker.ietf.org/doc/rfc5639
#[no_mangle]
pub static brainpoolP224r1: Ecc = Ecc { f: 224 };

/// Represents the curve brainpoolP256r1 specified in [RFC 5639].
///
/// [RFC 5639]: https://datatracker.ietf.org/doc/rfc5639
#[no_mangle]
pub static brainpoolP256r1: Ecc = Ecc { f: 256 };

/// Represents the curve brainpoolP320r1 specified in [RFC 5639].
///
/// [RFC 5639]: https://datatracker.ietf.org/doc/rfc5639
#[no_mangle]
pub static brainpoolP320r1: Ecc = Ecc { f: 320 };

/// Represents the curve brainpoolP384r1 specified in [RFC 5639].
///
/// [RFC 5639]: https://datatracker.ietf.org/doc/rfc5639
#[no_mangle]
pub static brainpoolP384r1: Ecc = Ecc { f: 384 };

/// Represents the curve brainpoolP512r1 specified in [RFC 5639].
///
/// [RFC 5639]: https://datatracker.ietf.org/doc/rfc5639
#[no_mangle]
pub static brainpoolP512r1: Ecc = Ecc { f: 512 };

/// Represents the curve secp256k1 specified in [SEC 2].
///
/// [SEC 2]: https://www.secg.org/sec2-v2.pdf
#[no_mangle]
pub static secp256k1: Ecc = Ecc { f: 256 };