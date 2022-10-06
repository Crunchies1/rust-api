use std::{io};
use actix_web::{App, HttpServer};
use actix_cors::Cors;

mod users;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .wrap(cors)
            // enable logger - always register actix-web Logger middleware last
            // .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(users::list)
            .service(users::get)
            .service(users::create)
            .service(users::delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
