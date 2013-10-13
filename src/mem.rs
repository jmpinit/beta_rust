pub struct Mem {
	data:	~[u8]
}

impl Mem {
	pub fn read(&self, addr: u32) -> u8 {
		self.data[addr]
	}

	pub fn read_u32(&self, addr: u32) -> u32 {
		let b1 = self.read(addr) as u32;
		let b2 = self.read(addr + 1) as u32;
		let b3 = self.read(addr + 2) as u32;
		let b4 = self.read(addr + 3) as u32;

		(b1 << 24) | (b2 << 16) | (b3 << 8) | b4
	}

	pub fn write(&mut self, addr: u32, val: u8) {
		self.data[addr] = val;
	}

	pub fn write_u32(&mut self, addr: u32, val: u32) {
		self.data[addr]		= (val >> 24 & 0xFF) as u8;
		self.data[addr + 1]	= (val >> 16 & 0xFF) as u8;
		self.data[addr + 2]	= (val >> 8 & 0xFF) as u8;
		self.data[addr + 3]	= (val & 0xFF) as u8;
	}
}