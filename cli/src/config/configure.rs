pub trait Configurable {
    type Out;
    fn configure(self) -> Self::Out;
}