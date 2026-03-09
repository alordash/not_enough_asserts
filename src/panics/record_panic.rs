use crate::panics::silent_unwind_catching::catch_unwind_silent;
use std::panic;
use crate::panics::panic_error_msg_extraction::extract_panic_error_msg;

/// Expects given closure to panic and returns panic message.
/// Closure's panic message must be either `String` or `&str`.
///
/// # Examples
/// ```
/// use not_enough_asserts::panics::record_panic;
///
/// let error_msg = record_panic(|| panic!("hello world!"));
/// assert_eq!("hello world!", error_msg);
/// ```
pub fn record_panic<T>(callback: impl FnOnce() -> T) -> String {
    let panic_error = catch_unwind_silent(panic::AssertUnwindSafe(callback));
    if let Some(actual_error) = panic_error.err().as_deref() {
        let actual_error_message = extract_panic_error_msg(actual_error);
        return actual_error_message;
    } else {
        panic!("Expected to panic.");
    }
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
        assert_eq!(error_msg, actual_error_msg)
    }

    #[test]
    fn record_panic_PanicsString_Ok() {
        // Arrange
        let error_msg = "quo vadis";

        // Act
        let actual_error_msg = record_panic(|| panic_any(String::from(error_msg)));

        // Assert
        assert_eq!(error_msg, actual_error_msg)
    }

    #[test]
    #[should_panic(expected = "Expected to panic.")]
    fn record_panic_DoesNotPanic_ExpectsToPanic() {
        // Arrange
        // Act
        record_panic(|| ());
    }

    #[test]
    #[should_panic(expected = "Panic's error should be either `String` or `&str`.")]
    fn record_panic_PanicErrorIsNotString_ExpectsString() {
        // Arrange
        // Act
        record_panic(|| panic_any(1));
    }
}
