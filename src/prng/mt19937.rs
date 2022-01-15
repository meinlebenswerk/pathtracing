use super::PRNG;

/*
Coefficients for  MT19937, stolen from https://en.wikipedia.org/wiki/Mersenne_Twister
(w, n, m, r) = (32, 624, 397, 31)
w = 32
n = 624
n = 397
r = 31
a = 9908B0DF16
(u, d) = (11, 0xFFFFFFFF)
(s, b) = (7, 0x9D2C5680)
(t, c) = (15, 0xEFC60000)
l = 18
*/


pub struct Mt19937Prng {
  state: Vec<u32>,
  index: usize,
  lower_mask: u32,
  upper_mask: u32
}

impl Mt19937Prng {
  pub fn new(seed: u32) -> Self {
    let mut state = vec![0; 624];
    state[0] = seed;

    let lower_mask = (1 << 30) - 1;
    let upper_mask = 1 << 31;


    let mut prng = Self {
      state,
      index: 0,
      lower_mask,
      upper_mask
    };
    prng.init();
    prng
  }

  fn init(&mut self) {
    for i in 1..623 {
      let previous = self.state[i-1] as u64;
      let val = (1812433253 * (previous ^ (previous >> 30))) & 0xFFFFFFFF;
      self.state[i] = val as u32 + i as u32;
    }
  }

  fn twist(&mut self) {
    for i in 0..623 {
      let x = (self.state[i] & self.upper_mask) + (self.state[(i + 1) % 623] & self.lower_mask);
      let mut x_a = x >> 1;
      if (x & 0x1) != 0 { x_a ^= 0x9908B0DF; }
      self.state[i] = self.state[(i + 397) % 623] ^ x_a;
    }
    self.index = 0;
  }

  pub fn next_u32(&mut self) -> u32 {
    if self.index == 624 {
      self.twist();
    }

    let mut y = self.state[self.index];
    y = y ^ ( y >> 11);
    y = y ^ ((y << 7)  & 0x9D2C5680);
    y = y ^ ((y << 15) & 0xEFC60000);
    y = y ^ ( y >> 1);
    self.index += 1;
    y
  }
}

impl PRNG for Mt19937Prng {
  fn next_f32(&mut self) -> f32 {
    let val_u32 = self.next_u32();
    val_u32 as f32 / std::u32::MAX as f32
  }
}