use std::vec;
use mem::Mem;

pub struct Beta {
	halted:		bool,
	pc:			u32,
	register:	[u32, ..31],
	mem:		Mem,

	// hidden registers
	op:			u32,
	data:		u32
}

impl Beta {
	pub fn new(size: uint) -> ~Beta {
		let new_data = vec::from_elem(size, 0u8);

		~Beta {
			halted: false,
			pc: 0u32,
			register: [0u32, ..31],
			mem: Mem {
				data: new_data
			},
			op: 0u32,
			data: 0u32
		}
	}

	pub fn reset(&mut self) {
		// clear registers
		for i in range(0, self.register.len()) {
			self.register[i] = 0;
		}

		// back to start
		self.pc = 0;

		// start
		self.halted = false;
	}

	pub fn tick(&mut self) {
		if !self.halted {
			let instruction: u32 = self.mem.read_u32(self.pc);

			// decode invariant part of instruction format
			self.op = instruction >> 26;
			self.data = instruction & 0x3FFFFFFF;

			//println(fmt!("op: 0x%x", op as uint));

			print(fmt!("%x: ", instruction as uint));
			self.execute();

			self.pc += 4;
			//println(fmt!("tick. pc at %d", self.pc as int));
		}
	}

	fn execute(&mut self) {
		match self.op {
			0x00 => { self.halted = true }

			// arithmetic
			0x20 => { self.exec_op("add",		|a, b| a + b); }
			0x21 => { self.exec_op("sub",		|a, b| a - b); }
			0x22 => { self.exec_op("mul",		|a, b| a*b); }
			0x23 => { self.exec_op("div",		|a, b| a / b); }
			// -- constant
			0x30 => { self.exec_op_c("addc",	|a, b| a + b); }
			0x31 => { self.exec_op_c("subc",	|a, b| a - b); }
			0x32 => { self.exec_op_c("mulc",	|a, b| a*b); }
			0x33 => { self.exec_op_c("divc",	|a, b| a / b); }

			// logic
			0x28 => { self.exec_op("and",		|a, b| a & b); }
			0x29 => { self.exec_op("or",		|a, b| a|b); }
			0x2A => { self.exec_op("nor",		|a, b| a^b); }
			0x2B => { self.exec_op("xnor",	|a, b| !(a^b)); }
			0x2C => { self.exec_op("shl",		|a, b| a << b); }
			0x2D => { self.exec_op("shr",		|a, b| a >> b); }
			0x2E => { self.exec_op("sra",		|a, b| ((a as i32) >> b) as u32); }
			// -- constant
			0x38 => { self.exec_op_c("andc", 	|a, b| a & b); }
			0x39 => { self.exec_op_c("orc",	|a, b| a|b); }
			0x3A => { self.exec_op_c("xorc",	|a, b| a^b); }
			0x3B => { self.exec_op_c("xnorc",	|a, b| !(a^b)); }
			0x3C => { self.exec_op_c("shlc",	|a, b| a << b); }
			0x3D => { self.exec_op_c("shrc",	|a, b| a >> b); }
			0x3E => { self.exec_op_c("srac",	|a, b| ((a as i32) >> b) as u32); }

			// compare
			0x24 => { self.exec_op("cmpeq",	|a, b| if(a == b)	{1} else {0}); }
			0x26 => { self.exec_op("cmple",	|a, b| if(a <= b)	{1} else {0}); }
			0x25 => { self.exec_op("cmplt",	|a, b| if(a < b)	{1} else {0}); }
			// -- constant
			0x34 => { self.exec_op_c("cmpeqc",	|a, b| if(a == b)	{1} else {0}); }
			0x36 => { self.exec_op_c("cmplec",	|a, b| if(a <= b)	{1} else {0}); }
			0x35 => { self.exec_op_c("cmpltc",	|a, b| if(a < b)	{1} else {0}); }

			// branch
			0x1B => {
				let (r_c, r_a, _lit) = Beta::arg_ptrs_c(self.data);
				println(fmt!("jmp %x, %x", r_c as uint, r_a as uint));

				self.write_reg(r_c as uint, self.pc + 4);
				self.pc = (self.read_reg(r_a as uint) -4) & 0xFFFFFFFC;
			}
			0x1C => {
				let (r_c, r_a, lit) = Beta::arg_ptrs_c(self.data);
				println(fmt!("beq %x, %x, %x", r_c as uint, r_a as uint, lit as uint));
				let a = self.read_reg(r_a as uint);

				self.write_reg(r_c as uint, self.pc + 4);
				let displacement = (lit as i16 as u32)*4;
				let target = self.pc + displacement;

				if(a == 0) { self.pc = target; }
			}
			0x1D => {
				let (r_c, r_a, lit) = Beta::arg_ptrs_c(self.data);
				println(fmt!("bne %x, %x, %x", r_c as uint, r_a as uint, lit as uint));
				let a = self.read_reg(r_a as uint);

				self.write_reg(r_c as uint, self.pc + 4);
				let displacement = (lit as i16 as u32)*4;
				let target = self.pc + displacement;

				if(a != 0) { self.pc = target; }

				// memory io
			}
			0x18 => {
				let (r_c, r_a, lit) = Beta::arg_ptrs_c(self.data);

				println(fmt!("ld %x, %x, %x", r_c as uint, r_a as uint, lit as uint));

				let a = self.read_reg(r_a as uint);

				let ea = a + (lit as i16 as u32);
				let val = self.mem.read_u32(ea as u32);

				self.write_reg(r_c as uint, val)
			}
			0x1F => {
				let (r_c, r_a, lit) = Beta::arg_ptrs_c(self.data);
				println(fmt!("ldr %x, %x, %x", r_c as uint, r_a as uint, lit as uint));
				let a = self.read_reg(r_a as uint);

				let ea = (self.pc & 0x7FFFFFFF) + (lit as i16 as u32)*4;
				let memval = self.mem.read_u32(ea);

				self.write_reg(r_c as uint, memval);

				if(a != 0b11111) { warn!("\"The Ra field is ignored and should be 11111.\" It is not."); }
			}
			0x19 => {
				let (r_c, r_a, lit) = Beta::arg_ptrs_c(self.data);
				let a = self.read_reg(r_a as uint);
				let c = self.read_reg(r_c as uint);

				let ea = a + (lit as i16 as u32);

				self.mem.write_u32(ea, c)

			}
			_ => fail!("Unrecognized opcode.")
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

	fn arg_ptrs(data: u32) -> (u32, u32, u32) {
		(data >> 21 & 0x1F, data >> 16 & 0x1F, data >> 11 & 0x1F)
	}

	fn arg_ptrs_c(data: u32) -> (u32, u32, u32) {
		(data >> 21 & 0x1F, data >> 16 & 0x1F, data & 0xFFFF)
	}

	fn arg_vals(&self, data: u32) -> (u32, u32) {
		let (_, r_a, r_b) = Beta::arg_ptrs(data);
		(self.read_reg(r_a as uint), self.read_reg(r_b as uint))
	}

	fn arg_vals_c(&self, data: u32) -> (u32, u32) {
		let (_, r_a, lit) = Beta::arg_ptrs_c(data);
		(self.read_reg(r_a as uint), lit)
	}

	fn exec_op_c(&mut self, name: &str, exp: &fn(a: u32, b: u32) -> u32) {
		let (a, b) = self.arg_vals_c(self.data);
		let (r_c, r_a, lit) = Beta::arg_ptrs(self.data);
		self.write_reg(r_c as uint, exp(a, b));

		// debug
		println(fmt!("%s\t%x, %x, %x", name, r_c as uint, r_a as uint, lit as uint));
	}

	fn exec_op(&mut self, name: &str, exp: &fn(a: u32, b: u32) -> u32) {
		let (a, b) = self.arg_vals(self.data);
		let (r_c, r_a, r_b) = Beta::arg_ptrs(self.data);
		self.write_reg(r_c as uint, exp(a, b));

		// debug
		println(fmt!("%s\t%x, %x, %x", name, r_c as uint, r_a as uint, r_b as uint));
	}
}

