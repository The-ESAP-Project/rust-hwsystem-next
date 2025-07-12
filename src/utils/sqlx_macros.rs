#[macro_export]
macro_rules! sqlx_enum_type {
    ($db:ty, $value_ref:ty, $struct:ty) => {
        impl Type<$db> for $struct {
            fn type_info() -> <$db as sqlx::Database>::TypeInfo {
                <String as Type<$db>>::type_info()
            }
        }

        impl<'r> Decode<'r, $db> for $struct {
            fn decode(value: $value_ref) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let s: String = <String as Decode<$db>>::decode(value)?;
                <$struct>::from_str(&s).map_err(|e| format!("UserRole decode error: {e}").into())
            }
        }
    };
}
