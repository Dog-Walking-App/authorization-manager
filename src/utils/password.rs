use argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();

  let hashed_password = argon2
    .hash_password(password.as_bytes(), &salt)?;
  
  Ok(hashed_password.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, argon2::password_hash::Error> {
  let parsed_hash = PasswordHash::new(&password_hash)?;
  Ok(Argon2::default()
    .verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
