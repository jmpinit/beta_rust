use std::vec;
use std::iter::Range;
use mem::{Mem, Word, Address};

type Literal = i16;
type Reg = u8;

type Instruction = u32;
//type ConstInstruction = Instruction;

trait Decodeable {
	fn opcode(&self) -> u32;	// TODO implemented by parent trait
	fn registers(&self) -> (Reg, Reg, Reg);
	fn destination(&self) -> Reg;
	fn literal(&self) -> Literal;
}

impl Decodeable for Instruction {
	fn literal(&self) -> Literal { (*self & 0xFFFF) as Literal }
	fn opcode(&self) -> u32 { *self as u32 >> 26 }

	fn destination(&self) -> Reg {
		let (dest, _, _) = self.registers();
		return dest;
	}

	fn registers(&self) -> (Reg, Reg, Reg) {
		let v = *self as u32;
		((v >> 21 & 0x1F) as u8, (v >> 16 & 0x1F) as u8, (v >> 11 & 0x1F) as u8)
	}
}

/*impl Decodeable for ConstInstruction {
	fn literal(&self) -> Literal { (*self as u32 & 0xFFFF) as Literal }

	fn registers(&self) -> (Reg, Reg, Reg) {
		let v = *self as u32;
		(*self >> 21 & 0x1F, v >> 16 & 0x1F, 0)
	}
}*/

pub struct Beta {
	halted:		bool,
	pc:			Address,
	register:	[u32, ..31],
	mem:		Mem,
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
			}
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
			let instruction = self.mem.read_word(self.pc);
			self.execute(instruction);
			self.pc += 4;
			self.dump_registers(range(0, 4));
			println("");
		}
	}

	fn execute(&mut self, instruction: Instruction) {
		let op_c = |name: &str, exp: &fn(a: Word, b: Word) -> Word| {
			let (_, a, _) = self.register_values(instruction);
			let b = instruction.literal();
			let (r_c, r_a, _) = instruction.registers();

			self.write_reg(r_c, exp(a, b as Word));
			println(fmt!("OP: %x <> %x = %x", a as uint, b as uint, exp(a, b as Word) as uint));

			Beta::dump_instruction_c(name, r_c, r_a, b);
		};

		let op = |name: &str, exp: &fn(a: Word, b: Word) -> Word| {
			let (_, a, b) = self.register_values(instruction);
			let (r_c, r_a, r_b) = instruction.registers();

			self.write_reg(r_c, exp(a, b));

			Beta::dump_instruction(name, r_c, r_a, r_b);
		};

		match instruction.opcode() {
			0x00 => { self.halted = true; println("HALT!"); }

			// arithmetic
			0x20 => { op("add",		|a, b| a + b); }
			0x21 => { op("sub",		|a, b| a - b); }
			0x22 => { op("mul",		|a, b| a*b); }
			0x23 => { op("div",		|a, b| a / b); }
			// -- constant
			0x30 => { op_c("addc",	|a, b| a + b); }
			0x31 => { op_c("subc",	|a, b| a - b); }
			0x32 => { op_c("mulc",	|a, b| a*b); }
			0x33 => { op_c("divc",	|a, b| a / b); }

			// logic
			0x28 => { op("and",		|a, b| a & b); }
			0x29 => { op("or",		|a, b| a|b); }
			0x2A => { op("nor",		|a, b| a^b); }
			0x2B => { op("xnor",	|a, b| !(a^b)); }
			0x2C => { op("shl",		|a, b| a << b); }
			0x2D => { op("shr",		|a, b| a >> b); }
			0x2E => { op("sra",		|a, b| a >> b); } // FIXME (correct sign?)
			// -- constant
			0x38 => { op_c("andc", 	|a, b| a & b); }
			0x39 => { op_c("orc",	|a, b| a | b); }
			0x3A => { op_c("xorc",	|a, b| a ^ b); }
			0x3B => { op_c("xnorc",	|a, b| !(a ^ b)); }
			0x3C => { op_c("shlc",	|a, b| a << b); }
			0x3D => { op_c("shrc",	|a, b| a >> b); }
			0x3E => { op_c("srac",	|a, b| a >> b); }	// FIXME (correct sign?)

			// compare
			0x24 => { op("cmpeq",	|a, b| if(a == b)	{1} else {0}); }
			0x26 => { op("cmple",	|a, b| if(a <= b)	{1} else {0}); }
			0x25 => { op("cmplt",	|a, b| if(a < b)	{1} else {0}); }
			// -- constant
			0x34 => { op_c("cmpeqc",	|a, b| if(a == b)	{1} else {0}); }
			0x36 => { op_c("cmplec",	|a, b| if(a <= b)	{1} else {0}); }
			0x35 => { op_c("cmpltc",	|a, b| if(a < b)	{1} else {0}); }

			// branch
			0x1B => {
				let (r_c, r_a, _) = instruction.registers();

				self.write_reg(r_c, self.pc + 4);
				self.pc = (self.read_reg(r_a ) - 4) & 0xFFFFFFFC;

				Beta::dump_instruction("jmp", r_c, r_a, 0xFF);
			}
			0x1C => {
				let (r_c, r_a, _) = instruction.registers();
				let lit = instruction.literal();
				//println(fmt!("beq %x, %x, %x", r_c as uint, r_a as uint, lit as uint));
				let a = self.read_reg(r_a);

				self.write_reg(r_c, self.pc + 4);
				let displacement = (lit*4) as Word;
				let target = self.pc + displacement;

				if(a == 0) { self.pc = target; }
				
				Beta::dump_instruction_c("beq", r_c, r_a, lit);
			}
			0x1D => {
				let (r_c, r_a, _) = instruction.registers();
				let lit = instruction.literal();
				let a = self.read_reg(r_a);

				self.write_reg(r_c, self.pc + 4);
				let displacement = (lit*4) as Word;
				let target = self.pc + displacement;

				if(a != 0) { self.pc = target; }

				Beta::dump_instruction_c("bne", r_c, r_a, lit);
			}

			// memory io
			0x18 => {
				let (r_c, r_a, _) = instruction.registers();
				let lit = instruction.literal();

				let a = self.read_reg(r_a);

				let val = self.mem.read_word(a + (lit as Word));

				self.write_reg(r_c, val);

				Beta::dump_instruction_c("ld", r_c, r_a, lit);
			}

			0x1F => {
				let (r_c, r_a, _) = instruction.registers();
				let lit = instruction.literal();
				let a = self.read_reg(r_a);

				let ea = (self.pc & 0x7FFFFFFF) + (lit*4) as Word;
				let memval = self.mem.read_word(ea);

				self.write_reg(r_c, memval);

				if(a != 0b11111) { warn!("\"The Ra field is ignored and should be 11111.\" It is not."); }
				
				Beta::dump_instruction_c("ldr", r_c, r_a, lit);
			}
			
			0x19 => {
				let (r_c, r_a, _) = instruction.registers();
				let lit = instruction.literal();
				let a = self.read_reg(r_a);
				let c = self.read_reg(r_c);

				let ea = a + lit as Word;

				self.mem.write_word(ea, c);

				Beta::dump_instruction_c("st", r_c, r_a, lit);
			}
			_ => fail!("Unrecognized opcode.")
		}
	}

	/* DEBUG */

	fn dump_instruction(name: &str, dest: Reg, r_a: Reg, r_b: Reg) {
		println(fmt!("%s\t%x, %x, %x", name, dest as uint, r_a as uint, r_b as uint));
	}

	fn dump_instruction_c(name: &str, dest: Reg, r_a: Reg, lit: Literal) {
		println(fmt!("%s\t%x, %x, %d(%x)", name, dest as uint, r_a as uint, lit as int, lit as uint));
	}

	pub fn dump_registers(&self, range: Range<int>) {
		//assert!(

		for i in range.clone() {
			print(fmt!("r%d\t|", i as int));
		}

		println("");

		for i in range.clone() {
			print(fmt!("%x\t", self.register[i] as uint));
		}

		println("");
	}

	fn dump_memory(&self) {
		for i in range(0, self.mem.data.len()) {
			print(fmt!("%x, ", self.mem.data[i] as uint));
		}
	}

	/* DECODE */

	fn register_values(&self, instruction: Instruction) -> (Word, Word, Word) {
		let (r_a, r_b, r_c) = instruction.registers();

		let c = self.read_reg(r_c);
		let a = self.read_reg(r_a);
		let b = self.read_reg(r_b);

		return (a, b, c);
	}

	/* REGISTERS */
	
	fn read_reg(&self, reg: Reg) -> Word {
		match reg {
			0..30	=> self.register[reg],
			31		=> 0,
			_		=> fail!(fmt!("tried to read from nonexistant register %d (0x%x)", reg as int, reg as uint))
		}
	}

	fn write_reg(&mut self, reg: Reg, val: Word) {
		println(fmt!("WRITE %d = %x", reg as int, val as uint));

		match reg {
			0..30	=> self.register[reg] = val,
			31		=> {},
			_		=> fail!(fmt!("tried to write to nonexistant register %d", reg as int))
		};
	}
}

