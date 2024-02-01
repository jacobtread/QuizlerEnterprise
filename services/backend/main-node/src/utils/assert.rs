/// If the condition is not true the `error` will be returned
#[inline]
#[must_use = "Assert does nothing if error is not used"]
pub fn assert<E>(condition: bool, error: E) -> Result<(), E> {
    if condition {
        Ok(())
    } else {
        Err(error)
    }
}
