mod args;

use args::Opt;
use structopt::StructOpt;

fn main() {
	let _args = Opt::from_args();
}
