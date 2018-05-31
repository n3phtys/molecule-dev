
use authentication::UserId;

pub type Action = &'static str;
pub type RoleId = i32;
pub type TableId = &'static str;
pub type EntityId = i32;

pub trait AuthorizationService {
    fn authorize(&mut self, user_id: UserId, role_id: &[RoleId], table_id : &TableId, entity_id : Option<&EntityId>) -> Result<Vec<Action>, ()>;
}

