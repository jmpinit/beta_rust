macro_rules! decode_op(
	($op:ident $handler:ident $data:ident) => (
		match $op {
			0x20 => $handler.add($data),
			0x30 => $handler.addc($data),
			0x28 => $handler.and($data),
			0x38 => $handler.andc($data),
			0x1C => $handler.beq($data),
			0x1D => $handler.bne($data),
			0x24 => $handler.cmpeq($data),
			0x34 => $handler.cmpeqc($data),
			0x26 => $handler.cmple($data),
			0x36 => $handler.cmplec($data),
			0x25 => $handler.cmplt($data),
			0x35 => $handler.cmpltc($data),
			0x23 => $handler.div($data),
			0x33 => $handler.divc($data),
			0x1B => $handler.jmp($data),
			0x18 => $handler.ld($data),
			0x1F => $handler.ldr($data),
			0x22 => $handler.mul($data),
			0x32 => $handler.mulc($data),
			0x29 => $handler.or($data),
			0x39 => $handler.orc($data),
			0x2C => $handler.shl($data),
			0x3C => $handler.shlc($data),
			0x2D => $handler.shr($data),
			0x3D => $handler.shrc($data),
			0x2E => $handler.sra($data),
			0x3E => $handler.srac($data),
			0x21 => $handler.sub($data),
			0x31 => $handler.subc($data),
			0x19 => $handler.st($data),
			0x2A => $handler.xor($data),
			0x3A => $handler.xorc($data),
			0x2B => $handler.xnor($data),
			0x3B => $handler.xnorc($data),
			_ => fail!("Unrecognized opcode.")
		}
)
)

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
		let data = instruction & 0x3FFFFFFF;

		println(fmt!("op: %d", op as int));

		decode_op!(op self data)

		self.pc += 4;
		//println(fmt!("tick. pc at %d", self.pc as int));
	}

	/* INSTRUCTION SET */
	
	fn add(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn addc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn and(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}

	fn andc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn beq(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn bne(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn cmpeq(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn cmpeqc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn cmple(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn cmplec(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn cmplt(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn cmpltc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn div(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn divc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn jmp(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn ld(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn ldr(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn mul(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn mulc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn or(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn orc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn shl(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn shlc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn shr(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn shrc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn sra(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn srac(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn sub(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn subc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn st(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn xor(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn xorc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn xnor(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
	}
	
	fn xnorc(&mut self, data: u32) {
		println(fmt!("data: %d", data as int));
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
