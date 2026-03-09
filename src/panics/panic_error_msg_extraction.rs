use std::any::Any;

pub(crate) const INVALID_PANIC_ERROR_TYPE_MSG: &'static str =
    "Panic's error should be either `String` or `&str`.";
pub(crate) fn extract_panic_error_msg(panic_error: &(dyn Any + Send)) -> String {
    let Some(actual_error_msg) = panic_error
        .downcast_ref::<String>()
        .map(String::clone)
        .or_else(|| panic_error.downcast_ref::<&str>().map(ToString::to_string))
    else {
        panic!("{}", INVALID_PANIC_ERROR_TYPE_MSG)
    };
    return actual_error_msg;
}
