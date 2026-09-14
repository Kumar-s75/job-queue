use tracing_subscriber::EnvFilter;
pub fn init(filter:&str){let _=tracing_subscriber::fmt().json().with_env_filter(EnvFilter::new(filter)).try_init();}
