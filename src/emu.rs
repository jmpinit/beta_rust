use std::os;
use std::io;
use std::path;

mod beta;

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
		let filename = args[1];
		let filedata = load(filename);

		let mut i = 0;
		while i < filedata.len() {
			print(fmt!("%x, ", filedata[i] as uint));
			i += 1;
		}
	} else {
		println("usage: emu <input file>");
	}

	let mut b = ~beta::Beta {
		pc: 10u32,
		register: [3u32, ..31],
		mem: beta::Mem {
			data: [0u8, ..4*1024]
		}
	};

	b.reset();
	b.tick();
	println("success");
}
