

type JWT = String;
pub type UserId = i32;

pub trait AuthenticationService {
    fn sign_up(&mut self, username: &str, password: &str) -> Result<JWT, ()>;

    fn sign_in(&mut self, username: &str, password: &str) -> Result<JWT, ()>;

    fn validate(&mut self, jwt: &JWT) -> Result<UserId, ()>;
}