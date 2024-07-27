pub struct Sha1;

//constants are derived from the fractional parts of the square roots of the first five primes
const H0: u32 = 0x67452301;
const H1: u32 = 0xEFCDAB89;
const H2: u32 = 0x98BADCFE;
const H3: u32 = 0x10325476;
const H4: u32 = 0xC3D2E1F0;

impl Default for Sha1 {
    fn default() -> Self {
        Self
    }
}
impl Sha1 {
    pub fn new() -> Self {
        Sha1
    }

    pub fn hash(&mut self, key: &str) -> [u8; 20] {
        // init with vars to the sha-1 initial hash value
        let (mut h0, mut h1, mut h2, mut h3, mut h4) = (H0, H1, H2, H3, H4);

        let msg = self.pad_message(key);

        for chunk in msg.chunks(64) {
            let (mut a, mut b, mut c, mut d, mut e) = (H0, H1, H2, H3, H4);
            let schedule = self.build_preliminary_schedule(chunk);

            for (i, schedule_val) in schedule.iter().enumerate() {
                let (f, k): (u32, u32) = match i {
                    0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                    20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                    _ => (b ^ c ^ d, 0xCA62C1D6),
                };
                let temp = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(*schedule_val);

                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }

        let mut hash = [0u8; 20];

        hash[0..4].copy_from_slice(&h0.to_be_bytes());
        hash[4..8].copy_from_slice(&h1.to_be_bytes());
        hash[8..12].copy_from_slice(&h2.to_be_bytes());
        hash[12..16].copy_from_slice(&h3.to_be_bytes());
        hash[16..].copy_from_slice(&h4.to_be_bytes());

        hash
    }

    fn build_preliminary_schedule(&mut self, chunk: &[u8]) -> [u32; 80] {
        let mut schedule = [0u32; 80];

        for (i, block) in chunk.chunks(4).enumerate() {
            schedule[i] = u32::from_be_bytes(block.try_into().unwrap());
        }

        for i in 16..80 {
            schedule[i] = schedule[i - 3] ^ schedule[i - 8] ^ schedule[i - 14] ^ schedule[i - 16];
            schedule[i] = schedule[i].rotate_left(1);
        }

        schedule
    }

    fn pad_message(&self, input: &str) -> Vec<u8> {
        let mut bytes = input.as_bytes().to_vec();
        // appending the bit-len apposed to the char len
        let original_bit_length = bytes.len() as u64 * 8;
        // 128 bits 0b1000000
        bytes.push(0x80);

        // 64-bits of the last 512 bits -> 512 - 64
        while (bytes.len() * 8) % 512 != 448 {
            bytes.push(0);
        }

        bytes.extend_from_slice(&original_bit_length.to_be_bytes());

        bytes
    }
}
