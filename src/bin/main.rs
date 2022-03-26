use auth::config;

#[rocket::launch]
async fn from_the_earth_to_the_moon() -> _ {
    config::app::build_rocket().await
}
