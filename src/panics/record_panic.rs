use crate::panics::silent_unwind_catcher::catch_unwind_silent;
use std::panic;

pub fn record_panic<T>(callback: impl FnOnce() -> T) -> String {
    let panic_error = catch_unwind_silent(panic::AssertUnwindSafe(callback));
    if let Some(actual_error) = panic_error.err().as_deref() {
        let error_message = actual_error
            .downcast_ref::<String>()
            .expect("Panic's error should be string.");
        return error_message.clone();
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
    fn record_panic_Panics_Ok() {
        // Arrange
        let error_msg = "quo vadis";

        // Act
        let actual_error_msg = record_panic(|| panic!("{error_msg}"));

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
    #[should_panic(expected = "Panic's error should be string.")]
    fn record_panic_PanicErrorIsNotString_ExpectsString() {
        // Arrange
        // Act
        record_panic(|| panic_any(1));
    }
}
