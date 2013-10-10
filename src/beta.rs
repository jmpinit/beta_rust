fn each(v: &mut [u32], op: &fn(v: &mut u32)) {
	let mut n = 0;
	while n < v.len() {
		op(&mut v[n]);
		n += 1;
	}
}

struct Beta {
	pc:			u32,
	register:	[u32, ..31],
	memory:		[u8, ..4096]
}

impl Beta {
	fn reset(&mut self) {
		self.pc = 0;
		do each(self.register) |r| { *r = 0; }

		//println(fmt!("reset. pc at %d", self.pc as int));
	}

	fn tick(&self) {
		//println(fmt!("tick. pc at %d", self.pc as int));

	}
}

fn main() {
	let mut b = ~Beta { pc: 10u32, register: [3u32, ..31], memory: [0u8, ..4*1024] };
	b.reset();
	b.tick();
	println("success");
}
