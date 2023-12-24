use actix_web::{
    web::{self},
    HttpResponse,
};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/status")
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
            .service(
                web::resource("/welcome")
                    .route(web::get().to(|| async { "Hello, world!" }))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            ),
    )
    .service(
        web::scope("/auth")
            .service(
                web::resource("/login")
                    .route(web::get().to(|| async { "Login page" }))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            )
            .service(
                web::resource("/signup")
                    .route(web::get().to(|| async { "Signup page" }))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
            ),
    );
}
