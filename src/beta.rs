struct Beta {
	pc:			u32,
	register:	~[u32],
	memory:		~[u8]
}

fn main() {
	let b = Beta { pc: 0u32, register: ~[0u32, ..31], memory: ~[0u8, ..4*1024] };
	println("success");
}
