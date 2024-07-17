#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    _banned: bool, // unused
    pub name: String, // no "@" symbol
    pub password_hash: String, // PHC string
}

impl User {
    pub fn new(user: &str, pwd: &str)->Self {
        Self {
            _banned: false, 
            name: user.to_string(), 
            password_hash: Self::hash_password(pwd), 
        }
    }

    pub fn _set_password(&mut self, password: &str) {
        self.password_hash = Self::hash_password(password);
    }

    pub fn verify_password(&self, password:&str)->bool{
        use sha_crypt::sha512_check;
        sha512_check(password, &self.password_hash).is_ok()
    }

    fn hash_password(pwd: &str)->String {
        use sha_crypt::{Sha512Params, sha512_simple};
        let params = Sha512Params::new(10_000).unwrap();
        sha512_simple(pwd, &params).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pwd(){
        let mut user = User::new("wsm", "114514");
        assert!(user.verify_password("114514"));
        user._set_password("1919810");
        assert!(user.verify_password("1919810"));
    }
}