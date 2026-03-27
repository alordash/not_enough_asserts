#[macro_export]
macro_rules! assert_type_eq {
    ($value:expr, $expected:ty) => {
        assert_eq!(
            core::any::type_name::<$expected>(),
            core::any::type_name_of_val($value)
        )
    };
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
        type ActualType = i32;
        type ExpectedType = String;
        let value: ActualType = Default::default();

        // Act
        let panic_msg = record_panic(|| assert_type_eq!(&value, ExpectedType));

        // Assert
        let expected_panic_msg = format!(
            "assertion `left == right` failed
  left: \"{}\"
 right: \"{}\"",
            core::any::type_name::<ExpectedType>(),
            core::any::type_name::<ActualType>()
        );
        assert_eq!(Some(expected_panic_msg), panic_msg);
    }
}
