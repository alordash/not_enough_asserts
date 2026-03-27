use crate::panics::silent_unwind_catching::catch_unwind_silent;
use std::panic;
use crate::panics::panic_error_msg_extraction::extract_panic_error_msg;

/// Returns panic message from given closure if it panics, otherwise returns `None`.
/// Closure's panic message must be either `String` or `&str`, otherwise downcasting will cause
/// another panic that won't be caught by this method.
///
/// # Examples
/// ```
/// use not_enough_asserts::panics::record_panic;
///
/// let error_msg = record_panic(|| panic!("hello world!"));
/// assert_eq!(Some("hello world!".to_owned()), error_msg);
/// ```
pub fn record_panic<T>(callback: impl FnOnce() -> T) -> Option<String> {
    let panic_error = catch_unwind_silent(panic::AssertUnwindSafe(callback));
    let actual_error_msg = panic_error.err().as_deref().map(extract_panic_error_msg);
    return actual_error_msg;
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use std::panic::panic_any;

    #[test]
    fn record_panic_PanicsStrRef_Ok() {
        // Arrange
        let error_msg = "quo vadis";

        // Act
        let actual_error_msg = record_panic(|| panic_any(error_msg));

        // Assert
        assert_eq!(Some(error_msg.to_owned()), actual_error_msg);
    }

    #[test]
    fn record_panic_PanicsString_Ok() {
        // Arrange
        let error_msg = "quo vadis".to_owned();

        // Act
        let actual_error_msg = record_panic(|| panic_any(error_msg.clone()));

        // Assert
        assert_eq!(Some(error_msg), actual_error_msg)
    }

    #[test]
    fn record_panic_DoesNotPanic_ReturnsNone() {
        // Arrange
        // Act
        let error_msg = record_panic(|| ());
        
        // Assert
        assert!(error_msg.is_none());
    }

    #[test]
    #[should_panic(expected = "Panic's error should be either `String` or `&str`.")]
    fn record_panic_PanicErrorIsNotString_ExpectsString() {
        // Arrange
        // Act
        record_panic(|| panic_any(1));
    }
}
