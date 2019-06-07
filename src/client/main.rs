use clap::{App, ArgMatches, AppSettings};
use std::env::args_os;
use tikv_client::Error;
use failure::{Fail, AsFail, Backtrace};
use ::log::{log, info, debug};

// TODO: Work off Futures 0.3 migration, use tokio runtime.
fn main() {
    // TODO: Init Environment/Config discovery.
    env_logger::try_init().ok();

    let mut app = Root::app();

    // Figure out the operating mode (REPL vs Oneshot)
    // TODO

    // Get matches
    let arg_matches = match app.get_matches_from_safe_borrow(args_os()) {
        // If the user input initially wrong arguments, we should print help.
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        Ok(arg_matches) => arg_matches,
    };
    println!("{:?}", arg_matches);
}

fn exit_with_backtrace(e: impl Fail) -> ! {
    info!("{:?}", e.as_fail());
    if let Some(backtrace) = e.backtrace() {
        debug!("{}", backtrace);
    }
    std::process::exit(1);
}

/// `Component`s are just reusable `clap::App`s with handlers. Just call them with `app` to get 
/// the Clap `App`/`Subcommand object. (Yes you can just use an `App`, cool huh?) then later use
/// `handle()` to chain the args in.
///
/// `Component`s are deliberately simple: They just take a `tikv_client::Client` and
/// the `clap::ArgMatches` when they handle the request. This means splitting one into its own app,
/// or integrating it with another `clap::App` is ridiculously simple. If you want tighter
/// integration please check out the library portion of the crate.
pub trait Component {
    /// Builds the `clap::App` that corresponds to this `Component`.
    fn app() -> App<'static, 'static>;

    /// Handle the request with some `ArgMatches`. This does one of two things:
    ///   1. It finalizes and terminates a request.
    ///   2. Invoke another `Component`. Eg. With `tikv-client raw get xyz` the
    ///      `raw::Root` component which would call the `raw::Get` component.
    // TODO: we may want to have some configuration passed around.
    fn handle(config: (), arg_matches: &ArgMatches) -> Result<(), Error>;
}

struct Root {}

impl Component for Root {
    fn app() -> App<'static, 'static> {
        App::new(env!("CARGO_PKG_NAME"))
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .global_settings(&[
                AppSettings::ColoredHelp,
                AppSettings::GlobalVersion,
                AppSettings::InferSubcommands,
            ])
            .setting(AppSettings::SubcommandRequired)
    }
    fn handle(config: (), arg_matches: &ArgMatches) -> Result<(), Error> {
        unimplemented!();
    }
}