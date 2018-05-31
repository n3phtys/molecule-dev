

type JWT = String;
pub type USER_ID = i32;

trait AuthenticationService {
    fn sign_up(username: &str, password: &str) -> Result<JWT, ()>;

    fn sign_in(username: &str, password: &str) -> Result<JWT, ()>;

    fn validate(jwt: &JWT) -> Result<USER_ID, ()>;
}