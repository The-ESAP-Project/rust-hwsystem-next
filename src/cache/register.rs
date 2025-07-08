use crate::cache::traits::ObjectCache;
use crate::errors::Result;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
};

pub type BoxedObjectCacheFuture =
    Pin<Box<dyn Future<Output = Result<Box<dyn ObjectCache>>> + Send>>;
pub type ObjectCacheConstructor = Arc<dyn Fn() -> BoxedObjectCacheFuture + Send + Sync>;

static OBJECT_CACHE_REGISTRY: Lazy<RwLock<HashMap<String, ObjectCacheConstructor>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn register_object_cache_plugin<S: Into<String>>(name: S, constructor: ObjectCacheConstructor) {
    let name = name.into();
    let mut registry = OBJECT_CACHE_REGISTRY.write().unwrap();
    registry.insert(name, constructor);
}

pub fn get_object_cache_plugin(name: &str) -> Option<ObjectCacheConstructor> {
    OBJECT_CACHE_REGISTRY.read().unwrap().get(name).cloned()
}

pub fn debug_object_cache_registry() {
    let registry = OBJECT_CACHE_REGISTRY.read().unwrap();
    if registry.is_empty() {
        tracing::debug!("No object cache plugins registered.");
    } else {
        tracing::debug!("Registered object cache plugins:");
        for key in registry.keys() {
            tracing::debug!(" - {}", key);
        }
    }
}
