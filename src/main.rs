use rocket::launch;
use structopt::StructOpt;

mod api;
mod config;

#[derive(Debug, StructOpt)]
struct Cli {
    /// config file
    #[structopt(long = "config-file", short = "c", default_value = "config.yaml")]
    config_file: String,
}

#[launch]
fn rocket() -> _ {
    std::env::var("RUST_LOG").map_err(|_| {
        std::env::set_var("RUST_LOG", "error,routes=info");
    }).unwrap_or_default();
    env_logger::init();

    let args = Cli::from_args();

    let config: config::Config = confy::load_path(std::path::Path::new(&args.config_file)).unwrap();

    api::init()
}
