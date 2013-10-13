extern mod extra;

use std::path;
use std::io;
use std::os;
use extra::getopts;

use beta::Beta;

pub mod beta;
pub mod mem;

fn load(filename: &str) -> ~[u8] {
	let read_result: Result<@Reader, ~str>;
	read_result = io::file_reader(~path::Path(filename));

	if read_result.is_ok() {
		let reader = read_result.unwrap();
		return reader.read_whole_stream();
	}

	println(fmt!("Error reading file: %?", read_result.unwrap_err()));
	return ~[];
}

fn do_work(inp: &str, out: Option<~str>) {
	println(inp);

	/*match out {
		None => { println("No Output"); }
		Some(x) => {
		}
	}*/

	// get user file
	let filedata = load(inp);

	// make a beta
	let mut b = Beta::new(4*1024);

	// put user data in beta
	// beta is big endian, file is little endian
	for i in range(0, filedata.len()) {
		let pos = i/4*4 + (3-i%4);
		b.mem.write(pos as u32, filedata[i]);
	}

	// start
	b.reset();

	let mut count = 0;
	while count < 1000 {
		b.tick();
		count += 1;
	}
}


fn print_usage(program: &str, _opts: &[getopts::Opt]) {
	println(fmt!("Usage: %s [options]", program));
	println("-o\t\tOutput");
	println("-h --help\tUsage");
}

fn main() {
	let args = os::args();

	let program = args[0].clone();

	let opts = ~[
		getopts::optopt("o"),
		getopts::optflag("h"),
		getopts::optflag("help")
	];

	let matches = match getopts::getopts(args.tail(), opts) {
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};

	if matches.opt_present("h") || matches.opt_present("help") {
		print_usage(program, opts);
		return;
	}

	let output = matches.opt_str("o");
	let input: &str = if !matches.free.is_empty() {
		matches.free[0].clone()
	} else {
		print_usage(program, opts);
		return;
	};
	
	do_work(input, output);
}
