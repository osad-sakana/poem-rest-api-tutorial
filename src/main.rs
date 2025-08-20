use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};

struct Api;

#[OpenApi]
impl Api{
    #[oai(path = "/hello", method = "get")]
    async fn hello(&self) -> PlainText<String> {
        PlainText("Hello, world!".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api, "Hello API", "1.0.0");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}