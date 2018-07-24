extern crate rtjv;
extern crate serde_json;

#[macro_use]
extern crate clap;
extern crate failure;

use std::fs::File;
use std::process;
use std::io::Read;

use failure::Error;

fn main() -> Result<(), Error> {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            clap::Arg::with_name("file")
                .value_name("FILE")
                .required(true)
                .help("TODO")
        )
        .get_matches();

    let rtj_file = matches.value_of("file").unwrap();
    let mut rtj_s = String::new();
    File::open(rtj_file)?.read_to_string(&mut rtj_s)?;

    let vr = rtjv::validate(&rtj_s)?;
    match vr {
        None => {
            println!("{} is valid RTJSON", rtj_file);
            process::exit(0)
        }
        Some(mut state) => {
            println!("{} is not valid RTJSON", rtj_file);
            println!();
            /*for e in errors {
                //println!("{}", e.description());
                let s = serde_json::to_string_pretty(&e)?;
                println!("{}", s);
            }*/

            //let s = serde_json::to_string_pretty(&state)?;
            //println!("{}", s);

            //let s = serde_json::to_string_pretty(&errors)?;
            //rtjv::prune_state(&mut state);
            //println!("Pruned errors:");
            //println!();

            println!("{}", rtjv::pretty_error(&state));

            println!();
            process::exit(1)
        }
    }
}

