#[derive(Debug, PartialEq, Eq, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum PublicKeyAlgorithm {
    /// RSA (Encrypt and Sign) [HAC]
    RSA = 1,
    /// DEPRECATED: RSA (Encrypt-Only) [HAC]
    RSAEncrypt = 2,
    /// DEPRECATED: RSA (Sign-Only) [HAC]
    RSASign = 3,
    /// Elgamal (Encrypt-Only) [ELGAMAL] [HAC]
    ElgamalSign = 16,
    /// DSA (Digital Signature Algorithm) [FIPS186] [HAC]
    DSA = 17,
    /// Elliptic Curve: RFC-6637
    ECDH = 18,
    /// ECDSA: RFC-6637
    ECDSA = 19,
    /// DEPRECATED: Elgamal (Encrypt and Sign)
    Elgamal = 20,
    /// Reserved for Diffie-Hellman (X9.42, as defined for IETF-S/MIME)
    DiffieHellman = 21,
    /// EdDSA (not yet assigned)
    EdDSA = 22,
    /// Private experimental range (from OpenGPG)
    // TODO: genenric Unknown(u8)
    Private100 = 100,
    Private101 = 101,
    Private102 = 102,
    Private103 = 103,
    Private104 = 104,
    Private105 = 105,
    Private106 = 106,
    Private107 = 107,
    Private108 = 108,
    Private109 = 109,
    Private110 = 110,
}
