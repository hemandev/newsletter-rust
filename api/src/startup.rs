use crate::routes::{health_check::health_check, subscriptions::subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub fn app(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[cfg(test)]
mod tests {
    use entity::subscriptions::Entity as Subscriptions;
    use sea_orm::{ConnectOptions, Database, EntityTrait};
    use std::net::TcpListener;
    use tokio::spawn;

    use crate::configurations::get_config;

    #[tokio::test]
    async fn health_check_works() {
        let address = spawn_app();
        let client = reqwest::Client::new();
        let path = format!("http://{address}/health_check");
        let response = client
            .get(&path)
            .send()
            .await
            .expect(&format!("Failed to execute request. {path}"));

        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }

    #[tokio::test]
    async fn subscribe_returns_200_for_valid_data() {
        let config = get_config().expect("Failed to read configuration.");
        let connection_url = config.database.get_connection_url();
        let db = Database::connect(connection_url)
            .await
            .expect("Error occured while connecting to database.");

        let db_data = Subscriptions::find()
            .all(&db)
            .await
            .expect("Error occured while querying database.");

            

        assert_eq!(db_data[0].email, "ursula_le_guin@gmail.com");
        assert_eq!(db_data[0].name, "le guin");

        let address = spawn_app();
        let client = reqwest::Client::new();
        let path = format!("http://{address}/subscriptions");
        let response = client
            .post(&path)
            .body("name=le%20guin&email=ursula_le_guin%40gmail.com")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
            .expect(&format!("Failed to execute request. {path}"));

        assert_eq!(200, response.status().as_u16());
    }

    #[tokio::test]
    async fn subscribe_returns_400_for_invalid_data() {
        let address = spawn_app();

        let error_cases = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_msg) in error_cases {
            let client = reqwest::Client::new();
            let path = format!("http://{address}/subscriptions");
            let response = client
                .post(&path)
                .body(invalid_body)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .send()
                .await
                .expect(&format!("Failed to execute request. {path}"));

            assert_eq!(
                400,
                response.status().as_u16(),
                "Did not send 400 on invalid data {}",
                error_msg
            );
        }
    }

    fn spawn_app() -> String {
        let address = "127.0.0.1:0";
        let listener = TcpListener::bind(address).expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let server = super::app(listener).expect("Error while binding address");
        spawn(server);
        format!("127.0.0.1:{port}")
    }
}
