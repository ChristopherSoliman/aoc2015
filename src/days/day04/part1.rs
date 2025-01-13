pub fn run() -> u32 {
    let input = "iwrupvqb";
    let mut i = 1;
    loop {
        let t = format!("{input}{i}");
        let hash = md5(&t);
        if hash.starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

fn md5(message: &str) -> String {
    let mut bytes = message.as_bytes().to_vec();
    let mut a0: u32 = 0x67452301; // A
    let mut b0: u32 = 0xefcdab89; // B
    let mut c0: u32 = 0x98badcfe; // C
    let mut d0: u32 = 0x10325476; // D

    let message_len: u64 = bytes.len() as u64 * 8_u64;
    bytes.push(0b10000000);
    while bytes.len() * 8 % 512 != 448 {
        bytes.push(0_u8);
    }
    let length_as_u8 = split_u64_to_u8(message_len);
    bytes.extend(length_as_u8);

    for chunk in bytes.chunks_exact_mut(64) {
        let words = u8_to_u32_chunks(chunk);
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..64 {
            let mut f: u32 = 0;
            let mut g: u32 = 0;
            if i < 16 {
                f = (b & c) | (!b & d);
                g = i;
            } else if i < 32 {
                f = (d & b) | (!d & c);
                g = (5 * i + 1) % 16;
            } else if i < 48 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else if i < 64 {
                f = c ^ (b | !d);
                g = (7 * i) % 16;
            }
            f = f
                .wrapping_add(a)
                .wrapping_add(K[i as usize])
                .wrapping_add(words[g as usize]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i as usize]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    format!(
        "{:08x}{:08x}{:08x}{:08x}",
        a0.swap_bytes(),
        b0.swap_bytes(),
        c0.swap_bytes(),
        d0.swap_bytes(),
    )
}

fn u8_to_u32_chunks(chunks: &mut [u8]) -> Vec<u32> {
    let mut out: Vec<u32> = vec![];

    let mut i = 0;
    while i + 3 < chunks.len() {
        let temp_arr = [chunks[i], chunks[i + 1], chunks[i + 2], chunks[i + 3]];
        out.push(u32::from_ne_bytes(temp_arr));
        i += 4;
    }
    out
}

fn split_u64_to_u8(a: u64) -> [u8; 8] {
    [
        a as u8,
        (a >> 8) as u8,
        (a >> 16) as u8,
        (a >> 24) as u8,
        (a >> 32) as u8,
        (a >> 40) as u8,
        (a >> 48) as u8,
        (a >> 56) as u8,
    ]
}
