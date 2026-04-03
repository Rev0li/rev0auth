use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|p| p.to_string())
        .map_err(|_| "password_hash_failed".to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn test_hash_password_produces_valid_argon2() {
        let password = "my-very-strong-password";
        let hashed = hash_password(password).expect("hash should succeed");

        assert!(hashed.starts_with("$argon2"));
        assert!(verify_password(password, &hashed));
    }

    #[test]
    fn test_hash_password_repeats_differently() {
        let password = "my-very-strong-password";
        let first = hash_password(password).expect("first hash should succeed");
        let second = hash_password(password).expect("second hash should succeed");

        assert_ne!(first, second);
        assert!(verify_password(password, &first));
        assert!(verify_password(password, &second));
    }

    #[test]
    fn test_verify_password_with_correct_password() {
        let password = "my-very-strong-password";
        let hashed = hash_password(password).expect("hash should succeed");

        assert!(verify_password(password, &hashed));
    }

    #[test]
    fn test_verify_password_with_wrong_password() {
        let password = "my-very-strong-password";
        let hashed = hash_password(password).expect("hash should succeed");

        assert!(!verify_password("wrong-password", &hashed));
    }

    #[test]
    fn test_verify_password_with_corrupted_hash() {
        assert!(!verify_password("any-password", "not-a-valid-argon2-hash"));
    }
}
