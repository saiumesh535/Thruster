use bytes::{BufMut, BytesMut};
use std::collections::HashMap;

use crate::app::App;

use crate::core::context::Context;
use crate::core::request::decode;
use crate::core::request::Request;
use crate::core::response::{Response, StatusMessage};

pub async fn request<T: Context<Response = Response> + Clone + Send + Sync, S: 'static + Send>(
    app: &App<Request, T, S>,
    method: &str,
    route: &str,
    headers: &[(&str, &str)],
    body: &str,
) -> TestResponse {
    let headers_mapped: Vec<String> = headers
        .iter()
        .map(|val| format!("{}: {}", val.0, val.1))
        .collect();
    let headers = headers_mapped.join("\n");
    let body = format!(
        "{} {} HTTP/1.1\nHost: localhost:8080\n{}\n\n{}",
        method, route.to_owned(), headers, body
    );

    let mut bytes = BytesMut::with_capacity(body.len());
    bytes.put(body.as_bytes());

    let request = decode(&mut bytes).unwrap().unwrap();
    let matched_route = app.resolve_from_method_and_path("GET", route.to_owned());
    let response = app.resolve(request, matched_route).await.unwrap();

    TestResponse::new(response)
}

pub async fn get<T: Context<Response = Response> + Clone + Send + Sync, S: 'static + Send>(
    app: &App<Request, T, S>,
    route: &str,
) -> TestResponse {
    let body = format!("GET {} HTTP/1.1\nHost: localhost:8080\n\n", route.to_owned());

    let mut bytes = BytesMut::with_capacity(body.len());
    bytes.put(body.as_bytes());

    let request = decode(&mut bytes).unwrap().unwrap();
    let matched_route = app.resolve_from_method_and_path("GET", route.to_owned());
    let response = app.resolve(request, matched_route).await.unwrap();

    TestResponse::new(response)
}

pub async fn delete<T: Context<Response = Response> + Clone + Send + Sync, S: 'static + Send>(
    app: &App<Request, T, S>,
    route: &str,
) -> TestResponse {
    let body = format!("DELETE {} HTTP/1.1\nHost: localhost:8080\n\n", route.to_owned());

    let mut bytes = BytesMut::with_capacity(body.len());
    bytes.put(body.as_bytes());

    let request = decode(&mut bytes).unwrap().unwrap();
    let matched_route = app.resolve_from_method_and_path("DELETE", route.to_owned());
    let response = app.resolve(request, matched_route).await.unwrap();

    TestResponse::new(response)
}

pub async fn post<T: Context<Response = Response> + Clone + Send + Sync, S: 'static + Send>(
    app: &App<Request, T, S>,
    route: &str,
    content: &str,
) -> TestResponse {
    let body = format!(
        "POST {} HTTP/1.1\nHost: localhost:8080\nContent-Length: {}\n\n{}",
        route,
        content.len(),
        content
    );

    let mut bytes = BytesMut::with_capacity(body.len());
    bytes.put(body.as_bytes());

    let request = decode(&mut bytes).unwrap().unwrap();
    let matched_route = app.resolve_from_method_and_path("POST", route.to_owned());
    let response = app.resolve(request, matched_route).await.unwrap();

    TestResponse::new(response)
}

pub async fn put<T: Context<Response = Response> + Clone + Send + Sync, S: 'static + Send>(
    app: &App<Request, T, S>,
    route: &str,
    content: &str,
) -> TestResponse {
    let body = format!(
        "PUT {} HTTP/1.1\nHost: localhost:8080\nContent-Length: {}\n\n{}",
        route,
        content.len(),
        content
    );

    let mut bytes = BytesMut::with_capacity(body.len());
    bytes.put(body.as_bytes());

    let request = decode(&mut bytes).unwrap().unwrap();
    let matched_route = app.resolve_from_method_and_path("PUT", route.to_owned());
    let response = app.resolve(request, matched_route).await.unwrap();

    TestResponse::new(response)
}

pub async fn patch<T: Context<Response = Response> + Clone + Send + Sync, S: 'static + Send>(
    app: &App<Request, T, S>,
    route: &str,
    content: &str,
) -> TestResponse {
    let body = format!(
        "PATCH {} HTTP/1.1\nHost: localhost:8080\nContent-Length: {}\n\n{}",
        route,
        content.len(),
        content
    );

    let mut bytes = BytesMut::with_capacity(body.len());
    bytes.put(body.as_bytes());

    let request = decode(&mut bytes).unwrap().unwrap();
    let matched_route = app.resolve_from_method_and_path("PATCH", route.to_owned());
    let response = app.resolve(request, matched_route).await.unwrap();

    TestResponse::new(response)
}

#[derive(Debug)]
pub struct TestResponse {
    pub body: String,
    pub headers: HashMap<String, String>,
    pub status: (String, u32),
}

impl TestResponse {
    fn new(response: Response) -> TestResponse {
        let mut headers = HashMap::new();
        let header_string = String::from_utf8(response.header_raw.to_vec()).unwrap();

        for header_pair in header_string.split("\r\n") {
            if !header_pair.is_empty() {
                let mut split = header_pair.split(':');
                let key = split.next().unwrap().trim().to_owned();
                let value = split.next().unwrap().trim().to_owned();

                headers.insert(key, value);
            }
        }

        TestResponse {
            body: String::from_utf8(response.response).unwrap(),
            headers,
            status: match response.status_message {
                StatusMessage::Ok => ("Ok".to_owned(), 200),
                StatusMessage::Custom(code, message) => (message, code),
            },
        }
    }
}
