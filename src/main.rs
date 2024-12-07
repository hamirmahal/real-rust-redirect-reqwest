#![warn(clippy::pedantic)]

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
macro_rules! print_with_time {
    ($($arg:tt)*) => {{
        let now = std::time::SystemTime::now();
        let since_the_epoch = now
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("Time went backwards");
        let seconds = since_the_epoch.as_secs();
        let hours = (seconds / 3600) % 24;
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;
        let formatted_time = format!("[{:02}:{:02}:{:02}] ", hours, minutes, seconds);
        println!("{formatted_time}{}", format!($($arg)*));
    }};
}

#[derive(serde::Deserialize, serde::Serialize, std::fmt::Debug)]
struct GoogleAnalyticsEvent {
    name: String,
    params: serde_json::Value,
}

#[derive(serde::Deserialize, std::fmt::Debug)]
struct AnalyticsRequest {
    client_id: String,
    events: Vec<GoogleAnalyticsEvent>,
}

#[derive(serde::Serialize)]
struct ProxyPayload<'a> {
    client_id: &'a str,
    events: &'a [GoogleAnalyticsEvent],
}

#[post("/")]
/// You can test this endpoint locally with the following command:
/// ```sh
/// curl -v 0.0.0.0:8080 \
///   -H "Content-Type: application/json" \
///   -d '{"client_id":"example_client_id","events":[{"name":"test","params":{"test":"test"}}]}'
/// ```
async fn proxy_handler(request: web::Json<AnalyticsRequest>) -> impl Responder {
    /// <https://analytics.google.com/analytics/web/?authuser=3#/a337297802p468185970/admin/streams/table/9962630900>
    const ENDPOINT: &str = "https://www.google-analytics.com/mp/collect";
    const API_SECRET: &str = "zh6RAkfmTauY1f09Aw61tQ";
    const ID: &str = "G-RXSN2PE45G";
    let url = format!("{ENDPOINT}?measurement_id={ID}&api_secret={API_SECRET}");
    let client = reqwest::Client::new();
    print_with_time!("Received request: {:#?}", request);
    let payload = ProxyPayload {
        client_id: &request.client_id,
        events: &request.events,
    };

    // Forward the request to Google Analytics.
    match client.post(&url).json(&payload).send().await {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().json("Request successful")
            } else {
                HttpResponse::BadGateway()
                    .body(format!("Request failed with status: {}", response.status()))
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {err}")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(proxy_handler))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
