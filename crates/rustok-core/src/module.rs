use async_trait::async_trait;

#[async_trait]
pub trait RusToKModule: Send + Sync {
    fn slug(&self) -> &'static str;

    fn name(&self) -> &'static str;

    fn version(&self) -> &'static str;

    fn dependencies(&self) -> &[&'static str] {
        &[]
    }

    async fn on_enable(&self) -> crate::Result<()> {
        Ok(())
    }

    async fn on_disable(&self) -> crate::Result<()> {
        Ok(())
    }
}
