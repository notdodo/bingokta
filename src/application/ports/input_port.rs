pub trait InputPort {
    async fn process(&self);
    // async fn process(&self) -> Result<(), Box<dyn std::error::Error>>;
}
