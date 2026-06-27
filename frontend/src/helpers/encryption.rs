// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rustle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rustle.  If not, see <https://www.gnu.org/licenses/>.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use blowfish::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use blowfish::Blowfish;

pub fn encrypt(data: &str) -> Result<String, String> {
    let key_bytes = crate::constants::config::STATS_MIGRATION_OBFUSCATION_KEY.as_bytes();
    let bf =
        Blowfish::<byteorder::BigEndian>::new_from_slice(key_bytes).map_err(|e| e.to_string())?;

    let mut bytes = Vec::with_capacity(data.len() + 8);
    bytes.extend_from_slice(data.as_bytes());
    let pad_len = (8 - (bytes.len() % 8)) % 8;
    bytes.resize(bytes.len() + pad_len, 0);

    for chunk in bytes.chunks_mut(8) {
        let block = blowfish::cipher::generic_array::GenericArray::from_mut_slice(chunk);
        bf.encrypt_block(block);
    }

    Ok(STANDARD.encode(&bytes))
}

pub fn decrypt(encoded: &str) -> Option<String> {
    let key_bytes = crate::constants::config::STATS_MIGRATION_OBFUSCATION_KEY.as_bytes();
    let bf = Blowfish::<byteorder::BigEndian>::new_from_slice(key_bytes).ok()?;

    let mut bytes = STANDARD.decode(encoded).ok()?;
    if bytes.len() % 8 != 0 {
        return None;
    }

    for chunk in bytes.chunks_mut(8) {
        let block = blowfish::cipher::generic_array::GenericArray::from_mut_slice(chunk);
        bf.decrypt_block(block);
    }

    while let Some(&0) = bytes.last() {
        bytes.pop();
    }

    String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `encrypt` followed by `decrypt` must yield the original input.
    /// This is a round-trip smoke test, not a security claim.
    #[test]
    fn roundtrip_recovers_plaintext() {
        let plaintext = "{\"games_won\":42,\"current_streak\":7}";
        let encoded = encrypt(plaintext).expect("encrypt should succeed");
        let decoded = decrypt(&encoded).expect("decrypt should succeed");
        assert_eq!(decoded, plaintext);
    }

    /// Two encryptions of the same plaintext produce the same ciphertext.
    /// This is intentional for Blowfish in ECB mode without an IV, and
    /// confirms the migration format is deterministic.
    #[test]
    fn encryption_is_deterministic() {
        let plaintext = "stats-blob";
        let a = encrypt(plaintext).unwrap();
        let b = encrypt(plaintext).unwrap();
        assert_eq!(a, b);
    }

    /// Decryption of a non-base64 input returns `None` (no panic).
    #[test]
    fn decrypt_rejects_non_base64() {
        assert!(decrypt("not-base64!@#").is_none());
    }

    /// Decryption of base64 data with a length that's not a multiple of
    /// the Blowfish block size (8 bytes) returns `None`.
    #[test]
    fn decrypt_rejects_misaligned_input() {
        // 5 bytes is not a multiple of 8.
        let bad = STANDARD.encode(b"12345");
        assert!(decrypt(&bad).is_none());
    }
}
