#![feature(rustc_private)]

use rustc_driver::Callbacks;
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_session;

pub struct ChirpCallbacks;

impl Callbacks for ChirpCallbacks {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.override_queries = Some(|sess, prov| {
            
        })
    }
}

fn main() {
    println!("Hello, world!");
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    let args = rustc_driver::args::raw_args(&early_dcx)
    .unwrap_or_else(|_| std::process::exit(rustc_driver::EXIT_FAILURE));

    let using_internal_features =
    rustc_driver::install_ice_hook("https://github.com/fee1-dead/chirp/issues/new", |_| ());

    rustc_driver::install_ctrlc_handler();
    rustc_driver::catch_with_exit_code(move || {
        rustc_driver::RunCompiler::new(&args, &mut ChirpCallbacks)
            .set_using_internal_features(using_internal_features)
            .run()
    });
}
