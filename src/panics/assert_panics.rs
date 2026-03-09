use crate::panics::silent_unwind_catching::catch_unwind_silent;
use std::panic;
use crate::panics::panic_error_msg_extraction::extract_panic_error_msg;

/// Expects given closure to panic and verifies it's panic message.
/// Closure's panic message must be either `String` or `&str`.
///
/// # Examples
/// ```
/// use not_enough_asserts::panics::assert_panics;
/// assert_panics(|| panic!("hello world"), "hello world");
/// ```
pub fn assert_panics<T>(callback: impl FnOnce() -> T, expected_error_message: impl AsRef<str>) {
    let panic_error = catch_unwind_silent(panic::AssertUnwindSafe(callback));
    if let Some(actual_error) = panic_error.err().as_deref() {
        let actual_error_message = extract_panic_error_msg(actual_error);
        let expected_error_message_str = expected_error_message.as_ref();
        if expected_error_message_str != actual_error_message {
            panic!(
                "Wrong panic message.
Expected:
{expected_error_message_str}
  Actual:
{actual_error_message}"
            );
        }
        assert_eq!(expected_error_message.as_ref(), actual_error_message);
    } else {
        panic!(
            "Expected to panic with following error message:
{}",
            expected_error_message.as_ref()
        );
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use crate::panics::record_panic;
    use std::panic::panic_any;

    #[test]
    fn assert_panics_PanicsStrRef_Ok() {
        // Arrange
        let error_msg = "quo vadis";

        // Act
        // Assert
        assert_panics(|| panic_any(error_msg), error_msg);
    }

    #[test]
    fn assert_panics_PanicsString_Ok() {
        // Arrange
        let error_msg = "quo vadis";

        // Act
        // Assert
        assert_panics(|| panic_any(String::from(error_msg)), error_msg);
    }

    #[test]
    fn assert_panics_DoesNotPanic_ExpectsToPanic() {
        // Arrange
        let error_msg = "quo vadis";

        // Act
        let actual_error_msg = record_panic(|| assert_panics(|| (), error_msg));

        // Act
        let expected_error_msg = format!(
            "Expected to panic with following error message:
{error_msg}"
        );
        assert_eq!(expected_error_msg, actual_error_msg);
    }

    #[test]
    fn assert_panics_PanicsWithDifferentErrorMessage_ExpectsCorrectErrorMessage() {
        // Arrange
        let expected_error_msg = "quo vadis";
        let unexpected_error_msg = "veridis quo";

        // Act
        let actual_error_msg =
            record_panic(|| assert_panics(|| panic!("{unexpected_error_msg}"), expected_error_msg));

        // Assert
        let expected_error_msg = format!(
            "Wrong panic message.
Expected:
{expected_error_msg}
  Actual:
{unexpected_error_msg}"
        );
        assert_eq!(expected_error_msg, actual_error_msg);
    }

    #[test]
    #[should_panic(expected = "Panic's error should be either `String` or `&str`.")]
    fn record_panic_PanicErrorIsNotString_ExpectsString() {
        // Arrange
        // Act
        assert_panics(|| panic_any(1), "whatever");
    }
}
