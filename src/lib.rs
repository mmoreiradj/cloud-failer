use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{self, TraceLayer},
};

pub fn init_tracing() {
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .json()
            .with_max_level(tracing::Level::INFO)
            .init();
    }
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
}
