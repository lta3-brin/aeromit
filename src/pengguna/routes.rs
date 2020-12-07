//! # Module Pengguna Route
//!
//! Module ini digunakan untuk kelola route pengguna melalui fungsi yang disediakan.
//! Fungsi ini menerima satu parameter yaitu [_ServiceConfig_](https://docs.rs/actix-web/3.2.0/actix_web/web/struct.ServiceConfig.html).
//! Gunakan fungsi tersebut sebagai parameter dari fungsi [configure](https://docs.rs/actix-web/3.2.0/actix_web/struct.App.html#method.configure).
//!
//! <br />
//!
//! # Contoh
//!
//! ```rust
//! use crate::pengguna::routes::pengguna_route;
//! ```
use actix_web::web;
use crate::pengguna::handlers::{
    create_user,
    get_users,
    get_user,
    update_user,
};

/// # Fungsi pengguna_route
/// Fungsi ini menerima satu masukan yaitu [_ServiceConfig_](https://docs.rs/actix-web/3.2.0/actix_web/web/struct.ServiceConfig.html)
/// dan digunakan kedalam fungsi [_configure_](https://docs.rs/actix-web/3.2.0/actix_web/struct.App.html#method.configure).
///
/// <br />
///
/// # Masukan
///
/// * `route` - variabel dengan tipe _ServiceConvig_. Dapat digunakan didalam fungsi untuk
/// menambah/menggabungkan route baru.
///
/// <br />
///
/// # Keluaran
///
/// * `void` - tidak ada return value dari fungsi ini.
///
/// <br />
///
/// # Contoh
///
/// ```rust
/// pub fn root_route(route: &mut ServiceConfig) {
///     route.service(handler)
///         .service(
///             web::scope("/v1")
///                 .configure(config_lain)
///         )
/// }
/// ```
///
pub fn pengguna_route(route: &mut web::ServiceConfig) {
    route
        .service(create_user::new)
        .service(get_users::all)
        .service(get_user::by_id)
        .service(update_user::save);
}
