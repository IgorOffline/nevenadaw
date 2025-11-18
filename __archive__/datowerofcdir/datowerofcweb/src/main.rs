use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9112")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(
        "\
        <!doctype html>\
        <html lang=\"en\">\
        <head>\
          <meta charset=\"UTF-8\">\
          <meta content=\"width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0\" name=\"viewport\">\
          <meta content=\"ie=edge\" http-equiv=\"X-UA-Compatible\">\
          <title>datowerofcweb 0.1.0</title>\
        </head>\
        <body>\
          <h1>datowerofcweb 0.1.0</h1>\
        </body>\
        </html>\
        ",
    )
}
