use dotenvy::dotenv;

pub fn init_env() {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
}
