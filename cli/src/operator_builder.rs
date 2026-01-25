use opendal::Operator;

pub trait OperatorBuilder {
    fn build(&self) -> anyhow::Result<Operator>;
}
