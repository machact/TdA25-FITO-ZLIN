use actix_web::{middleware::NormalizePath, web};

use crate::api;

pub fn api_config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/api")
        .wrap(NormalizePath::new(
            actix_web::middleware::TrailingSlash::Trim, // Adjust behavior as needed
        ))
        .service(web::resource("")
            .route(web::get().to(api::hello_api))
        )
        .service(web::scope("/v1/games")
            .service(web::resource("")
                .route(web::get().to(api::games::get))
                .route(web::post().to(api::games::post))
            )
            .service(web::resource("/{uuid}")
                .route(web::get().to(api::games_uuid::get))
                .route(web::delete().to(api::games_uuid::delete))
                .route(web::put().to(api::games_uuid::put))
            )
        )
    );
}