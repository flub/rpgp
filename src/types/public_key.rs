use std::io;

use rand::{CryptoRng, Rng};

use crypto::hash::HashAlgorithm;
use errors::Result;
use types::{KeyTrait, Mpi};

pub trait PublicKeyTrait: KeyTrait {
    /// Verify a signed message.
    /// Data will be hashed using `hash`, before verifying.
    fn verify_signature(&self, hash: HashAlgorithm, data: &[u8], sig: &[Mpi]) -> Result<()>;

    /// Encrypt the given `plain` for this key.
    fn encrypt<R: CryptoRng + Rng>(&self, rng: &mut R, plain: &[u8]) -> Result<Vec<Mpi>>;

    // TODO: figure out a better place for this
    /// This is the data used for hashing in a signature. Only uses the public portion of the key.
    fn to_writer_old(&self, writer: &mut impl io::Write) -> Result<()>;
}

impl<'a, T: PublicKeyTrait> PublicKeyTrait for &'a T {
    fn verify_signature(&self, hash: HashAlgorithm, data: &[u8], sig: &[Mpi]) -> Result<()> {
        (*self).verify_signature(hash, data, sig)
    }

    fn encrypt<R: CryptoRng + Rng>(&self, rng: &mut R, plain: &[u8]) -> Result<Vec<Mpi>> {
        (*self).encrypt(rng, plain)
    }

    fn to_writer_old(&self, writer: &mut impl io::Write) -> Result<()> {
        (*self).to_writer_old(writer)
    }
}
