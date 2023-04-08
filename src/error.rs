use std::alloc::LayoutError;

/// Used ass a error Parameter in [`Array`](crate::Array) operations.
#[derive(Debug)]
pub struct ArrayError(pub String);

impl From<LayoutError> for ArrayError {

    #[inline]
    fn from(err: LayoutError) -> Self {
        ArrayError(err.to_string())
    }
}
