#[macro_export]
macro_rules! sqlx_enum_type {
    ($db:ty, $value_ref:ty, $struct:ty) => {
        impl sqlx::Type<$db> for $struct {
            fn type_info() -> <$db as sqlx::Database>::TypeInfo {
                <String as sqlx::Type<$db>>::type_info()
            }
        }

        impl<'r> sqlx::Decode<'r, $db> for $struct {
            fn decode(value: $value_ref) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let s: String = <String as sqlx::Decode<$db>>::decode(value)?;
                <$struct>::from_str(&s).map_err(|e| format!("UserRole decode error: {e}").into())
            }
        }
    };
}
