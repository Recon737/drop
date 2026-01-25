
use async_trait::async_trait;
use opendal::Operator;

#[async_trait]
pub trait OperatorBuilder {
    fn build(&self) -> anyhow::Result<Operator>;
}
