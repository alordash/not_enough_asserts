/// Asserts that `$value`'s type is equal to the `$expected` type.  
/// Types are compared using their [`TypeId`][core::any::TypeId].  
/// In error logs type names are printed using [`type_name`][core::any::type_name] and [`type_name_of_val`][core::any::type_name_of_val].
///
/// # Examples
///
/// Valid assertion.
/// ```
/// not_enough_asserts::assert_type_eq!(&0, i32);
/// ```
///
/// Failing assertion.
/// ```
/// use not_enough_asserts::prelude::*;
/// use core::any::TypeId;
///
/// let expected_error_msg = format!(
///     r#"type assertion failed
///  expected {:?}: "bool"
///    actual {:?}: "i32""#,
///     TypeId::of::<bool>(),
///     TypeId::of::<i32>());
///
/// assert_panics(|| assert_type_eq!(&0, bool), expected_error_msg);
/// ```
#[macro_export]
macro_rules! assert_type_eq {
    ($value:expr, $expected:ty) => {{
        fn get_type_id_of_value<T>(_: &T) -> core::any::TypeId {
            typeid::of::<T>()
        }

        let expected_type_id = typeid::of::<$expected>();
        let actual_type_id = get_type_id_of_value($value);
        if actual_type_id != expected_type_id {
            let expected_type_name = core::any::type_name::<$expected>();
            let actual_type_name = core::any::type_name_of_val($value);
            panic!(
                "type assertion failed
 expected {expected_type_id:?}: \"{expected_type_name}\"
   actual {actual_type_id:?}: \"{actual_type_name}\""
            )
        }
    }};
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use crate::panics::record_panic;

    #[test]
    fn assert_type_eq_SameTypes_Ok() {
        // Arrange
        type Type = i32;
        let value: Type = Default::default();
        // Act
        assert_type_eq!(&value, Type);
    }

    #[test]
    fn assert_type_eq_DifferentTypes_Panics() {
        // Arrange
        type ExpectedType = Box<String>;
        type ActualType = Box<i32>;
        let value: ActualType = Default::default();

        // Act
        let panic_msg = record_panic(|| assert_type_eq!(&value, ExpectedType));

        // Assert
        let expected_type_id = typeid::of::<ExpectedType>();
        let expected_type_name = core::any::type_name::<ExpectedType>();
        let actual_type_id = typeid::of::<ActualType>();
        let actual_type_name = core::any::type_name::<ActualType>();
        let expected_panic_msg = format!(
            "type assertion failed
 expected {expected_type_id:?}: \"{expected_type_name}\"
   actual {actual_type_id:?}: \"{actual_type_name}\""
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }

    #[test]
    fn assert_type_eq_DifferentTypesWithSameName_Panics() {
        // Arrange
        mod expected {
            pub struct Struct;
        }
        mod actual {
            pub struct Struct;
        }
        type ExpectedType = expected::Struct;
        type ActualType = actual::Struct;
        let value: ActualType = actual::Struct;

        // Act
        let panic_msg = record_panic(|| assert_type_eq!(&value, ExpectedType));

        // Assert
        let expected_type_id = typeid::of::<ExpectedType>();
        let expected_type_name = core::any::type_name::<ExpectedType>();
        let actual_type_id = typeid::of::<ActualType>();
        let actual_type_name = core::any::type_name::<ActualType>();
        let expected_panic_msg = format!(
            "type assertion failed
 expected {expected_type_id:?}: \"{expected_type_name}\"
   actual {actual_type_id:?}: \"{actual_type_name}\""
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }
}
