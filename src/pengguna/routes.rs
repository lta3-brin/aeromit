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
    create_user, get_users,
    get_user, update_user,
    delete_user, login_user,
    check_user, get_me
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
        .service(
            web::resource("/pengguna/")
                .name("create_get")
                .route(web::post().to(create_user::new))
                .route(web::get().to(get_users::all))
        )
        .service(login_user::masuk)
        .service(check_user::is_authenticated)
        .service(check_user::is_admin)
        .service(
            web::resource("/pengguna/me/")
                .name("get_me")
                .route(web::get().to(get_me::me_ok))
        )
        .service(
            web::resource("/pengguna/{id}/")
                .name("more_on_user")
                .route(web::get().to(get_user::by_id))
                .route(web::put().to(update_user::save))
                .route(web::delete().to(delete_user::by_id))
        );
}
