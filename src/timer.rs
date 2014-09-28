pub struct Timer {
	divider: u8,
	counter: u8,
	modulo: u8,
	enabled: bool,
	step: uint,
	internalcnt: uint,
	internaldiv: uint,
	pub interrupt: u8,
}

impl Timer {
	pub fn new() -> Timer {
		Timer {
			divider: 0,
			counter: 0,
			modulo: 0,
			enabled: false,
			step: 256,
			internalcnt: 0,
			internaldiv: 0,
			interrupt: 0,
		}
	}

	pub fn rb(&self, a: u16) -> u8 {
		match a {
			0xFF04 => self.divider,
			0xFF05 => self.counter,
			0xFF06 => self.modulo,
			0xFF07 => {
				(if self.enabled { 0x4 } else { 0 }) |
				(match self.step { 8 => 1, 32 => 2, 128 => 3, _ => 0 })
			}
			_ => fail!("Timer does not handler read {:4X}", a),
		}
	}

	pub fn wb(&mut self, a: u16, v: u8) {
		match a {
			0xFF04 => { self.divider = 0; },
			0xFF05 => { self.counter = v; },
			0xFF06 => { self.modulo = v; },
			0xFF07 => {
				self.enabled = v & 0x4 != 0;
				self.step = match v & 0x3 { 1 => 8, 2 => 32, 3 => 128, _ => 512 };
			},
			_ => fail!("Timer does not handler write {:4X}", a),
		};
	}

	pub fn do_cycle(&mut self, ticks: uint) {
		self.internaldiv += ticks;
		while self.internaldiv >= 128 {
			self.divider += 1;
			self.internaldiv -= 128;
		}

		if self.enabled {
			self.internalcnt += ticks;

			while self.internalcnt >= self.step {
				self.counter += 1;
				if self.counter == 0 {
					self.counter = self.modulo;
					self.interrupt |= 0x04;
				}
				self.internalcnt -= self.step;
			}
		}
	}
}

