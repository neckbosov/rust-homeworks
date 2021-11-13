pub use derive_builder::*;
pub trait Builder {
    type BuilderType;
    fn builder() -> Self::BuilderType;
}