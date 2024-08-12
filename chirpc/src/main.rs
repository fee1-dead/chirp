#![feature(rustc_private)]

use std::mem::replace;
use std::sync::OnceLock;

use rustc_driver::Callbacks;
use rustc_lint_defs::RegisteredTools;
use rustc_middle::ty::TyCtxt;
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;
use rustc_span::symbol::Ident;
use rustc_span::{Symbol, DUMMY_SP};

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_middle;
extern crate rustc_lint_defs;

pub struct ChirpCallbacks;

impl Callbacks for ChirpCallbacks {
    fn config(&mut self, config: &mut rustc_interface::interface::Config) {
        config.override_queries = Some(|_sess, prov| {
            // take the original query.
            static ORIG: OnceLock<fn(TyCtxt<'_>, ()) -> RegisteredTools> = OnceLock::new();
            ORIG.set(replace(&mut prov.registered_tools, |_, _| unreachable!())).unwrap();
            fn new(tcx: TyCtxt<'_>, (): ()) -> RegisteredTools {
                let mut set = (ORIG.get().unwrap())(tcx, ());
                // add "ti" as a registered tool.
                set.insert(Ident::new(Symbol::intern("ti"), DUMMY_SP));
                set
            }
            prov.registered_tools = new;
        })
    }
}

fn main() {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    let mut args = rustc_driver::args::raw_args(&early_dcx)
    .unwrap_or_else(|_| std::process::exit(rustc_driver::EXIT_FAILURE));

    args.remove(0);

    let using_internal_features =
    rustc_driver::install_ice_hook("https://github.com/fee1-dead/chirp/issues/new", |_| ());

    rustc_driver::install_ctrlc_handler();
    rustc_driver::catch_with_exit_code(move || {
        rustc_driver::RunCompiler::new(&args, &mut ChirpCallbacks)
            .set_using_internal_features(using_internal_features)
            .run()
    });
}
