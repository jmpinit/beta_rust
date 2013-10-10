fn each(v: &mut [u32], op: &fn(v: &mut u32)) {
	let mut n = 0;
	while n < v.len() {
		op(&mut v[n]);
		n += 1;
	}
}

struct Mem {
	data:	[u8, ..4096]
}

impl Mem {
	fn read(&self, addr: u32) -> u8 {
		self.data[addr]
	}

	fn read_word(&self, addr: u32) -> u32 {
		let b1 = self.read(addr) as u32;
		let b2 = self.read(addr + 1) as u32;
		let b3 = self.read(addr + 2) as u32;
		let b4 = self.read(addr + 3) as u32;

		(b1 << 24) | (b2 << 16) | (b3 << 8) | b4
	}

	fn write(&mut self, addr: u32, val: u8) {
		self.data[addr] = val;
	}
}

struct Beta {
	pc:			u32,
	register:	[u32, ..31],
	mem:		Mem
}

impl Beta {
	fn reset(&mut self) {
		self.pc = 0;
		do each(self.register) |r| { *r = 0; }

		//println(fmt!("reset. pc at %d", self.pc as int));
	}

	fn tick(&mut self) {
		let instruction: u32 = self.mem.read_word(self.pc);

		// decode invariant part of instruction format
		let op = instruction >> 26;
		let r_c = instruction >> 21;
		let r_a = instruction >> 16;

		println(fmt!("op: %d", op as int));

		match op {
			0x20 => println("ADD"),
			0x30 => println("ADDC"),
			0x28 => println("AND"),
			0x38 => println("ANDC"),
			0x1C => println("BEQ"),
			0x1D => println("BNE"),
			0x24 => println("CMPEQ"),
			0x34 => println("CMPEQC"),
			0x26 => println("CMPLE"),
			0x36 => println("CMPLEC"),
			0x25 => println("CMPLT"),
			0x35 => println("CMPLTC"),
			0x23 => println("DIV"),
			0x33 => println("DIVC"),
			0x1B => println("JMP"),
			0x18 => println("LD"),
			0x1F => println("LDR"),
			0x22 => println("MUL"),
			0x32 => println("MULC"),
			0x29 => println("OR"),
			0x39 => println("ORC"),
			0x2C => println("SHL"),
			0x3C => println("SHLC"),
			0x2D => println("SHR"),
			0x3D => println("SHRC"),
			0x2E => println("SRA"),
			0x3E => println("SRAC"),
			0x21 => println("SUB"),
			0x31 => println("SUBC"),
			0x19 => println("ST"),
			0x2A => println("XOR"),
			0x3A => println("XORC"),
			0x2B => println("XNOR"),
			0x3B => println("XNORC"),
			_ => fail!("Unrecognized opcode.")
		}

		self.pc += 4;
		//println(fmt!("tick. pc at %d", self.pc as int));
	}
}

fn main() {
	let mut b = ~Beta {
		pc: 10u32,
		register: [3u32, ..31],
		mem: Mem {
			data: [0u8, ..4*1024]
		}
	};

	b.reset();
	b.tick();
	println("success");
}
