#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use serde_json::json;
    use crate::rocket;

    #[test]
    fn test_registration() {
        use rocket::local::blocking::Client;

        let client = Client::tracked(rocket()).unwrap();


        let response = client.post("/api-v1/registration")
            .header(ContentType::JSON)
            .body(json!({
                "username": "arkady",
                "login": "arkady@mail.ru",
                "password": "qwerty12",
            }).to_string())
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}