use std::env;

use brainf_ck::cli::Cli;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli = match Cli::new(args) {
        Ok(cli) => cli,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    if let Err(err) = cli.run() {
        println!("{}", err)
    }
}
