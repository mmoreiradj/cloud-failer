use super::args::Args;

pub struct AppState {
    pub pool: sqlx::PgPool,
    pub app_config: Args,
}
