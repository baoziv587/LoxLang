extern crate clap;

mod commands;

use clap::{Arg, App, SubCommand};

#[macro_use]
extern crate log;

fn main() {
    #[cfg(feature = "pretty-env-logger")]
        pretty_env_logger::init();
    #[cfg(not(feature = "pretty-env-logger"))]
        env_logger::init();

    let matches = App::new("Lox Language")
        .version("0.1")
        .about("Lox language rust implementation")
        .subcommand(
            SubCommand::with_name("gen-ast")

                .arg_from_usage(
                    "-d, --dist=[PATH] 'Sets the input file to use'"
                )
                .about("Generate AST files automatically ")
                .help("You should set destination dir")
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("gen-ast") {
        if matches.is_present("dist") {
            let dist = matches.value_of("dist").unwrap();
            info!("[Run AST-generator] Path=>{} ", dist);
            commands::gen_ast::exec(dist);
        } else {
            error!("You should set destination dir");
        }
    }
//    info!("gen-ast : dist dir :[{}]", dist_dir);
}