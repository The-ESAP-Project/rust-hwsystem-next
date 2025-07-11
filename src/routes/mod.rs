pub mod auth;

pub mod users;

pub mod files;

pub mod system;
pub mod homeworks;

pub use auth::configure_auth_routes;
pub use files::configure_file_routes;
pub use system::configure_system_routes;
pub use users::configure_user_routes;
pub use homeworks::configure_homeworks_routes;
