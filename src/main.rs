use clap::{App, Arg, SubCommand};
use slog::{o, warn, Drain};

mod server;

use bragi_graphql::error;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let matches = App::new("Microservice for bragi graphql api")
        .version("0.1")
        .author("Matthieu Paindavoine")
        .subcommand(
            SubCommand::with_name("run")
                .about("Publish GraphQL API for Bragi")
                .version("0.1")
                .author("Matthieu Paindavoine <matt@area403.org>")
                .arg(
                    Arg::with_name("config")
                        .value_name("DIRECTORY")
                        .short("c")
                        .long("config")
                        .help("Config directory"),
                )
                .arg(
                    Arg::with_name("settings")
                        .value_name("NAME")
                        .short("s")
                        .long("settings")
                        .help("Settings used"),
                )
                .arg(
                    Arg::with_name("address")
                        .value_name("HOST")
                        .short("h")
                        .long("host")
                        .help("Address serving this server"),
                )
                .arg(
                    Arg::with_name("port")
                        .value_name("PORT")
                        .short("p")
                        .long("port")
                        .help("Port"),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Test Something")
                .version("0.1")
                .author("Matthieu Paindavoine <matt@area403.org>"),
        )
        .get_matches();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    match matches.subcommand() {
        ("run", Some(sm)) => server::run(sm, logger).await,
        // ("test", Some(sm)) => test::test(sm, logger).await,
        _ => {
            warn!(logger, "Unrecognized subcommand");
            Err(error::Error::MiscError {
                details: String::from("Unrecognized subcommand"),
            })
        }
    }
}
