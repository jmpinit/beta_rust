extern mod extra;

use std::path::Path;
use std::os;
use std::io;

use extra::fileinput::*;

fn main() {
	let args : ~[~str] = os::args();
	
	if args.len() == 2 {
		let source = FileInput::from_args();

		if Path(args[1]).exists() {
			let mut i = 0;
			do source.each_line |line| {
				if !line.is_empty() {
					io::println(fmt!("%d: %s", i, line));
					i += 1;
					true
				} else {
					false
				}
			};
		} else {
			println("error: file does not exist.");
		}
	} else {
		println("usage: uasm <input file>");
	}
}
