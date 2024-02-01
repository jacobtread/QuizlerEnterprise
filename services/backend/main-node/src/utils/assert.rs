/// If the condition is not true the `error` will be returned
#[inline]
pub fn assert<E>(condition: bool, error: E) -> Result<(), E> {
    if condition {
        Ok(())
    } else {
        Err(error)
    }
}
