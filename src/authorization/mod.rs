
use authentication::USER_ID;

pub type Action = &'static str;
pub type Role_Id = i32;
pub type Table_Id = &'static str;
pub type Entity_Id = i32;

trait AuthorizationService {
    fn authorize(user_id: USER_ID, roleId: &[Role_Id], table_id : &Table_Id, entity_id : Option<&Entity_Id>) -> Result<Vec<Action>, ()>;
}

