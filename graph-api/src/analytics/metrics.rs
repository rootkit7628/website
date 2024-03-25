use axum::{extract::MatchedPath, extract::Request, middleware::Next, response::IntoResponse};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

use std::time::Instant;


const QUERY_DURATION: &str = "query_duration";

pub(crate) fn create_recorderer() -> PrometheusHandle {

    const EXPO_SEC: &[f64] = &[
        0.0001, 0.001, 0.01, 0.1, 0.25, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
    .set_buckets_for_metric(Matcher::Full(QUERY_DURATION.to_string()), EXPO_SEC)
    .unwrap_or_else(|_| {
        panic!("Failed to set buckets for metric {}", QUERY_DURATION)
    })
    .install_recorder()
    .expect("Failed to install recorder")
}

pub(crate) async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_string()
    } else {
       req.uri().path().to_owned()
    };

    let method = req.method().clone();

    let response = next.run(req).await;
    let _latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status)
    ];

    metrics::counter!("http_requests_total", &labels);
    metrics::histogram!(QUERY_DURATION, "service" => "http");

    response
}
