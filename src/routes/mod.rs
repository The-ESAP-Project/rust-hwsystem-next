pub mod auth;

pub mod users;

pub mod classes;

pub mod files;

pub mod homeworks;

pub mod system;

pub use auth::configure_auth_routes;
pub use classes::configure_classes_routes;
pub use files::configure_file_routes;
pub use homeworks::configure_homeworks_routes;
pub use system::configure_system_routes;
pub use users::configure_user_routes;
