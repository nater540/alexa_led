// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate serde_derive;

embed_migrations!("migrations");

use clap::{App, Arg};
use dotenv::dotenv;

pub mod errors {
  use error_chain::*;

  error_chain! {
    foreign_links {
      Config(::config::ConfigError);
      Log(::log::SetLoggerError);
      Io(::std::io::Error);
    }
  }
}

use errors::*;

pub mod settings;
pub mod server;
pub mod alexa;
pub mod db;

use settings::Settings;

fn setup_logger() -> Result<()> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        record.target(),
        record.level(),
        message
      ))
    }).level(log::LevelFilter::Debug)
    .level_for("tokio_reactor", log::LevelFilter::Warn)
    .level_for("actix_web::server::server", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .chain(fern::log_file("output.log")?)
    .apply()?;

  Ok(())
}

fn main() {
  if let Err(ref err) = run() {
    use std::io::Write;

    let stderr = &mut std::io::stderr();
    let errmsg = "Error writing to stderr";

    writeln!(stderr, "Ragnarok encountered one or more errors:").expect(errmsg);
    for err in err.iter().skip(1) {
      writeln!(stderr, "  - {}", err).expect(errmsg);
    }

    if let Some(backtrace) = err.backtrace() {
      writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    }

    std::process::exit(1);
  }
}

fn run() -> Result<()> {
  dotenv().ok();

  let arguments = App::new("Ragnarok")
    .about("LED Strip Manager")
    .version("1.0")
    .arg(
      Arg::with_name("config")
        .long("config")
        .short("c")
        .value_name("FILE")
        .help("Sets a custom config file")
        .takes_value(true)
    ).get_matches();

  // Figure out what config file to load
  let cwd = ::std::env::current_dir()?;
  let default_config = format!("{}/config.yaml", cwd.display());
  let config_file = arguments.value_of("config").unwrap_or(&default_config);

  let settings = Settings::new(config_file)?;

  setup_logger()?;

  let sys = actix::System::new("ragnarok");
  server::start(&settings)?;
  let _ = sys.run();

  Ok(())
}
