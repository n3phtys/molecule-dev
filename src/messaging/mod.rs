use authentication::UserId;

pub trait Messager {
    fn send_message(&mut self, title: &str, content: &str, receivers: &[UserId]) -> Result<(), ()>;
}