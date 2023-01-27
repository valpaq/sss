mod input_output;
use clap::Parser;
use input_output::*;
use sss::*;
use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    #[arg(short, long, value_name = "command")]
    command: String,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    read_from: PathBuf,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    write_to: Option<PathBuf>,
}

fn main() {
    // let secret = 123435651345673413445647234u128.to_bigint().unwrap();
    // let res = split(&secret, 10, 3);

    // for (index, key_part) in res.clone().into_iter().enumerate() {
    //     println!("{}  {:x}", index + 1, key_part);
    // }

    // let res_secret = recover(vec![(1, res[0].clone()), (2, res[1].clone()), (3, res[2].clone())]);
    // println!("{res_secret:x} ");
    let cli = Cli::parse();
    match cli.command.as_str() {
        "split" => {
            let (key, n_parts, k_parts) = parse_split(cli.read_from);
            let res = split(&key, n_parts, k_parts);
            if let Some(write_to) = cli.write_to {
                let mut file = File::options()
                    .append(true)
                    .open(write_to)
                    .expect("no file");
                split_write(&mut file, res).expect("cannot write to file");
            } else {
                split_write(&mut io::stdout(), res).expect("cannot write to stdout");
            }
        }
        "recover" => {
            let points = parse_recover(cli.read_from);
            let res = recover(points);
            // println!("{:?}", res.to_str_radix(16).chars().step_by(2).map(|x| ));
            let string_to_print = recover_parse_result(&res);
            // let convert = convert_recover_to_str(res);
            if let Some(write_to) = cli.write_to {
                let mut file = File::options()
                    .append(true)
                    .open(write_to)
                    .expect("no file");
                recover_write(&mut file, &string_to_print).expect("cannot write to file");
            } else {
                recover_write(&mut io::stdout(), &string_to_print).expect("cannot write to stdout");
            }
        }
        _ => {}
    };
}
