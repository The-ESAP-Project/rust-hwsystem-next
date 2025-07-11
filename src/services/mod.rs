pub mod auth;
pub mod files;
pub mod homeworks;
pub mod system;
pub mod users;

pub use auth::AuthService;
pub use files::FileService;
pub use system::SystemService;
pub use users::UserService;
