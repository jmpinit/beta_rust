macro_rules! decode_op(
	($op:ident $handler:ident $data:ident) => (
		match $op {
			0x00 => $handler.halt(),
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

pub struct Mem {
	data:	[u8, ..4096]
}

impl Mem {
	fn read(&self, addr: u32) -> u8 {
		self.data[addr]
	}

	fn read_u32(&self, addr: u32) -> u32 {
		let b1 = self.read(addr) as u32;
		let b2 = self.read(addr + 1) as u32;
		let b3 = self.read(addr + 2) as u32;
		let b4 = self.read(addr + 3) as u32;

		(b1 << 24) | (b2 << 16) | (b3 << 8) | b4
	}

	pub fn write(&mut self, addr: u32, val: u8) {
		self.data[addr] = val;
	}

	fn write_u32(&mut self, addr: u32, val: u32) {
		self.data[addr]		= (val >> 24 & 0xFF) as u8;
		self.data[addr + 1]	= (val >> 16 & 0xFF) as u8;
		self.data[addr + 2]	= (val >> 8 & 0xFF) as u8;
		self.data[addr + 3]	= (val & 0xFF) as u8;
	}
}

pub struct Beta {
	halted:		bool,
	pc:			u32,
	register:	[u32, ..31],
	mem:		Mem
}

impl Beta {
	pub fn reset(&mut self) {
		self.pc = 0;
		do each(self.register) |r| { *r = 0; }

		self.halted = false;

		//println(fmt!("reset. pc at %d", self.pc as int));
	}

	pub fn tick(&mut self) {
		if !self.halted {
			let instruction: u32 = self.mem.read_u32(self.pc);

			// decode invariant part of instruction format
			let op = instruction >> 26;
			let data = instruction & 0x3FFFFFFF;

			//println(fmt!("op: 0x%x", op as uint));

			print(fmt!("%x: ", instruction as uint));
			decode_op!(op self data)

			self.pc += 4;
			//println(fmt!("tick. pc at %d", self.pc as int));
		}
	}

	pub fn dump_registers(&self) {
		for i in range(0, self.register.len()) {
			println(fmt!("r%d = %x", i as int, self.register[i] as uint));
		}
	}

	fn dump(&self) {
		for i in range(0, self.mem.data.len()) {
			print(fmt!("%x, ", self.mem.data[i] as uint));
		}
	}

	fn read_reg(&self, reg: uint) -> u32 {
		match reg {
			0..30	=> self.register[reg],
			31		=> 0,
			_		=> fail!(fmt!("tried to read from nonexistant register %d (0x%x)", reg as int, reg as uint))
		}
	}

	fn write_reg(&mut self, reg: uint, val: u32) {
		match reg {
			0..30	=> self.register[reg] = val,
			31		=> {},
			_		=> fail!(fmt!("tried to write to nonexistant register %d", reg as int))
		};
	}

	fn args(data: u32) -> (u32, u32, u32) {
		(data >> 21 & 0x1F, data >> 16 & 0x1F, data >> 11 & 0x1F)
	}

	fn args_literal(data: u32) -> (u32, u32, u32) {
		(data >> 21 & 0x1F, data >> 16 & 0x1F, data & 0xFFFF)
	}

	fn exec_op(&mut self, data: u32, exp: &fn(a: u32, b: u32) -> u32) {
		let (r_c, r_a, r_b) = Beta::args(data);
		println(fmt!("%x, %x, %x", r_c as uint, r_a as uint, r_b as uint));
		let a = self.read_reg(r_a as uint);
		let b = self.read_reg(r_b as uint);
		self.write_reg(r_c as uint, exp(a, b));
	}

	fn exec_op_lit(&mut self, data: u32, exp: &fn(a: u32, b: u32) -> u32) {
		let (r_c, r_a, lit) = Beta::args_literal(data);
		println(fmt!("%x, %x, %x", r_c as uint, r_a as uint, lit as uint));
		let a = self.read_reg(r_a as uint);
		self.write_reg(r_c as uint, exp(a, lit));
	}

	/* INSTRUCTION SET */

	fn halt(&mut self) {
		self.halted = true;
	}

	fn add(&mut self, data: u32) {
		print("add ");
		self.exec_op(data, |a, b| a + b);
	}
	
	fn addc(&mut self, data: u32) {
		print("addc ");
		self.exec_op_lit(data, |a, b| a + b);
	}
	
	fn and(&mut self, data: u32) {
		print("and ");
		self.exec_op(data, |a, b| a & b);
	}

	fn andc(&mut self, data: u32) {
		print("andc ");
		self.exec_op_lit(data, |a, b| a & b);
	}
	
	fn beq(&mut self, data: u32) {
		let (r_c, r_a, lit) = Beta::args_literal(data);
		println(fmt!("beq %x, %x, %x", r_c as uint, r_a as uint, lit as uint));
		let a = self.read_reg(r_a as uint);

		self.write_reg(r_c as uint, self.pc + 4);
		let displacement = (lit as i16 as u32)*4;
		let target = self.pc + displacement;

		if(a == 0) { self.pc = target; }
	}
	
	fn bne(&mut self, data: u32) {
		let (r_c, r_a, lit) = Beta::args_literal(data);
		println(fmt!("bne %x, %x, %x", r_c as uint, r_a as uint, lit as uint));
		let a = self.read_reg(r_a as uint);

		self.write_reg(r_c as uint, self.pc + 4);
		let displacement = (lit as i16 as u32)*4;
		let target = self.pc + displacement;

		if(a != 0) { self.pc = target; }
	}
	
	fn cmpeq(&mut self, data: u32) {
		print("cmpeq ");
		self.exec_op(data, |a, b| if(a == b) {1} else {0});
	}
	
	fn cmpeqc(&mut self, data: u32) {
		print("cmpeqc ");
		self.exec_op_lit(data, |a, b| if(a == b) {1} else {0});
	}
	
	fn cmple(&mut self, data: u32) {
		print("cmple ");
		self.exec_op(data, |a, b| if(a <= b) {1} else {0});
	}
	
	fn cmplec(&mut self, data: u32) {
		print("cmplec ");
		self.exec_op_lit(data, |a, b| if(a <= b) {1} else {0});
	}
	
	fn cmplt(&mut self, data: u32) {
		print("cmplt ");
		self.exec_op(data, |a, b| if(a < b) {1} else {0});
	}
	
	fn cmpltc(&mut self, data: u32) {
		print("cmpltc ");
		self.exec_op_lit(data, |a, b| if(a < b) {1} else {0});
	}
	
	fn div(&mut self, data: u32) {
		print("div ");
		self.exec_op(data, |a, b| a / b);
	}
	
	fn divc(&mut self, data: u32) {
		print("divc ");
		self.exec_op_lit(data, |a, b| a / b);
	}
	
	fn jmp(&mut self, data: u32) {
		let (r_c, r_a, _lit) = Beta::args_literal(data);
		println(fmt!("jmp %x, %x", r_c as uint, r_a as uint));

		self.write_reg(r_c as uint, self.pc + 4);
		self.pc = (self.read_reg(r_a as uint) -4) & 0xFFFFFFFC;
	}
	
	fn ld(&mut self, data: u32) {
		let (r_c, r_a, lit) = Beta::args_literal(data);

		println(fmt!("ld %x, %x, %x", r_c as uint, r_a as uint, lit as uint));

		let a = self.read_reg(r_a as uint);

		let ea = a + (lit as i16 as u32);
		let val = self.mem.read_u32(ea as u32);

		self.write_reg(r_c as uint, val)
	}
	
	fn ldr(&mut self, data: u32) {
		print("ldr ");
		let (r_c, r_a, lit) = Beta::args_literal(data);
		println(fmt!("ldr %x, %x, %x", r_c as uint, r_a as uint, lit as uint));
		let a = self.read_reg(r_a as uint);

		let ea = (self.pc & 0x7FFFFFFF) + (lit as i16 as u32)*4;
		let memval = self.mem.read_u32(ea);

		self.write_reg(r_c as uint, memval);

		if(a != 0b11111) { warn!("\"The Ra field is ignored and should be 11111.\" It is not."); }
	}
	
	fn mul(&mut self, data: u32) {
		print("mul ");
		self.exec_op(data, |a, b| a*b)
	}
	
	fn mulc(&mut self, data: u32) {
		print("mulc ");
		self.exec_op_lit(data, |a, b| a*b)
	}
	
	fn or(&mut self, data: u32) {
		print("or ");
		self.exec_op(data, |a, b| a|b)
	}
	
	fn orc(&mut self, data: u32) {
		print("orc ");
		self.exec_op_lit(data, |a, b| a|b)
	}
	
	fn shl(&mut self, data: u32) {
		print("shl ");
		self.exec_op(data, |a, b| a << b)
	}
	
	fn shlc(&mut self, data: u32) {
		print("shlc ");
		self.exec_op_lit(data, |a, b| a << b)
	}
	
	fn shr(&mut self, data: u32) {
		print("shr ");
		self.exec_op(data, |a, b| a >> b)
	}
	
	fn shrc(&mut self, data: u32) {
		print("shrc ");
		self.exec_op_lit(data, |a, b| a >> b)
	}
	
	fn sra(&mut self, data: u32) {
		print("sra ");
		self.exec_op(data, |a, b| ((a as i32) >> b) as u32)
	}
	
	fn srac(&mut self, data: u32) {
		print("srac ");
		self.exec_op_lit(data, |a, b| ((a as i32) >> b) as u32)
	}
	
	fn sub(&mut self, data: u32) {
		print("sub ");
		self.exec_op(data, |a, b| a - b)
	}
	
	fn subc(&mut self, data: u32) {
		print("subc ");
		self.exec_op_lit(data, |a, b| a - b)
	}
	
	fn st(&mut self, data: u32) {
		print("st ");
		let (r_c, r_a, lit) = Beta::args_literal(data);
		let a = self.read_reg(r_a as uint);
		let c = self.read_reg(r_c as uint);

		let ea = a + (lit as i16 as u32);

		self.mem.write_u32(ea, c)
	}
	
	fn xor(&mut self, data: u32) {
		print("xor ");
		self.exec_op(data, |a, b| a^b);
	}
	
	fn xorc(&mut self, data: u32) {
		print("xorc ");
		self.exec_op_lit(data, |a, b| a^b);
	}
	
	fn xnor(&mut self, data: u32) {
		print("xnor ");
		self.exec_op(data, |a, b| !(a^b));
	}
	
	fn xnorc(&mut self, data: u32) {
		print("xnorc ");
		self.exec_op_lit(data, |a, b| !(a^b));
	}
}

