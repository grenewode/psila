#![no_std]

/// Crypto backend error codes
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    /// Operation is not implemented
    NotImplemented,
    /// The key have a invalid size
    InvalidKeySize,
    /// The key has invalid data
    InvalidKey,
    /// The IV has a invalid size
    InvalidIvSize,
    /// The IV has invalid data
    InvalidIv,
    /// The data has a invalid size
    InvalidDataSize,
    /// The nonce has a invalid size
    InvalidNonceSize,
    /// The nonce has invalid data
    InvalidNonce,
    /// The additional data has invalid size
    InvalidAdditionalDataSize,
    /// The additional data has invalid data
    InvalidAdditionalData,
    /// The message integrity code (MIC) has invalid size
    InvalidIntegrityCodeSize,
    /// The message integrity code (MIC) has invalid data
    InvalidIntegrityCode,
    /// The message integrity code (MIC) check failed
    IntegrityCheckFailed,
    /// Other error, probably a error code from the backend
    Other(u32),
}

/// Trait for implemeting a crypto backend
pub trait CryptoBackend {
    /// Encrypt using CCM*
    fn ccmstar_encrypt(
        &mut self,
        // The Key to be used
        key: &[u8],
        // Nonce
        nonce: &[u8],
        // Clear test message
        message: &[u8],
        // Length of the message integrity code (MIC)
        mic_length: usize,
        // Additional data
        additional_data: &[u8],
        // Encrypted message
        message_output: &mut [u8],
    ) -> Result<usize, Error>;

    /// Decrypt using CCM*
    fn ccmstar_decrypt(
        &mut self,
        // The Key to be used
        key: &[u8],
        // Nonce
        nonce: &[u8],
        // Encrypted message with message integrity code (MIC)
        message: &[u8],
        // Length of the message integrity code (MIC)
        mic_length: usize,
        // Additional data
        additional_data: &[u8],
        // Clear text message
        message_output: &mut [u8],
    ) -> Result<usize, Error>;

    /// Set the key
    fn aes128_ecb_encrypt_set_key(&mut self, key: &[u8]) -> Result<(), Error>;
    /// Set the IV
    fn aes128_ecb_encrypt_set_iv(&mut self, iv: &[u8]) -> Result<(), Error>;
    /// Process blocks of data
    fn aes128_ecb_encrypt_process_block(
        &mut self,
        input: &[u8],
        output: &mut [u8],
    ) -> Result<(), Error>;
    /// Process the last bits and bobs and finish
    fn aes128_ecb_encrypt_finish(&mut self, input: &[u8], output: &mut [u8]) -> Result<(), Error>;
}