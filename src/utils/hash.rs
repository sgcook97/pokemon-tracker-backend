use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_pw(pw: String) -> String {
    let hashed_pw = hash(&pw, DEFAULT_COST).unwrap();
    hashed_pw
}

pub fn verify_pw(pw: String, hashed_pw: String) -> Result<bool, bcrypt::BcryptError> {
    return verify(&pw, &hashed_pw);
}
