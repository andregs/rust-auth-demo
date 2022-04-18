use double_checked_cell_async::DoubleCheckedCell;
use lazy_static::lazy_static;
use rocket::async_test;
use rocket::http::{Status, ContentType};
use rocket::local::asynchronous::Client;
use uuid::{Uuid, Variant::RFC4122};

use rust_auth_demo::{config, model::*};

lazy_static! {
    static ref CLIENT: DoubleCheckedCell<Client> = DoubleCheckedCell::new();
}

async fn get_client() -> &'static Client {
    // TODO automate the clearing of test database before each execution
    std::env::set_var("APP_PROFILE", "test");
    CLIENT
        .get_or_init(async {
            let server = config::app::build_rocket().await;
            Client::tracked(server)
                .await
                .expect("valid rocket instance")
        })
        .await
}

#[async_test]
async fn it_should_execute_e2e_happy_path() {
    let (username, password) = ("foo", "bar12345");
    let client = get_client().await;
    
    // registration
    
    let body = format!(r#"{{ "username": "{}", "password": "{}" }}"#, username, password);
    let res = client.post("/register").body(&body).header(ContentType::JSON).dispatch().await;
    assert_eq!(res.status(), Status::Created);
    
    let location = res.headers().get_one("Location").expect("location header was expected");
    assert!(location.starts_with("/profile/"));
    
    let new_id = location.split("/").last().expect("generated id was expected");
    let new_id = new_id.parse::<u64>().expect("numeric id was expected");
    assert!(new_id > 0);

    // login
    
    let res = client.post("/login").body(&body).header(ContentType::JSON).dispatch().await;
    assert_eq!(res.status(), Status::Ok);
    
    let actual = res.into_json::<LoginOk>().await.expect("login response was expected");
    let token = Uuid::parse_str(&actual.token).expect("uuid token was expected");
    assert_eq!(token.get_variant(), RFC4122);

    // authentication

    let res = client.post("/authenticate").body(token.to_string()).header(ContentType::Plain).dispatch().await;
    assert_eq!(res.status(), Status::Ok);

    let actual = res.into_json::<AuthOk>().await.expect("auth response was expected");
    assert_eq!(username, actual.username);
}
