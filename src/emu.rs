use std::os;
use std::io;
use std::path;
use beta::Beta;

pub mod beta;
pub mod mem;

fn load(filename: ~str) -> ~[u8] {
	let read_result: Result<@Reader, ~str>;
	read_result = io::file_reader(~path::Path(filename));

	if read_result.is_ok() {
		let reader = read_result.unwrap();
		return reader.read_whole_stream();
	}

	println(fmt!("Error reading file: %?", read_result.unwrap_err()));
	return ~[];
}

fn main() {
	let args : ~[~str] = os::args();
	
	if args.len() == 2 {
		// make a beta
		let mut b = Beta::new(4*1024);

		// get user file
		let filename = args[1];
		let filedata = load(filename);

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
	} else {
		println("usage: emu <input file>");
	}
}
