pub trait CacheAccessor {
    fn store(&mut self, key: &str, value: &str) -> ();
}