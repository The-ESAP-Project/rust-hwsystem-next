#[macro_export]
macro_rules! declare_object_cache_plugin {
    ($name:expr, $ty:ty) => {
        #[ctor::ctor]
        fn __register_cache_plugin() {
            use std::sync::Arc;
            use $crate::cache::register::register_object_cache_plugin;

            register_object_cache_plugin(
                $name,
                Arc::new(|| {
                    Box::pin(async {
                        match <$ty>::new() {
                            Ok(cache) => Ok(Box::new(cache) as Box<dyn $crate::cache::ObjectCache>),
                            Err(e) => Err($crate::errors::HWSystemError::cache_connection(e)),
                        }
                    })
                }),
            );
        }
    };
}
