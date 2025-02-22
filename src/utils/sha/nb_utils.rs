pub const sha1_init:[u32; 5] = [
            0x67452301,
            0xefcdab89,
            0x98badcfe,
            0x10325476,
            0xc3d2e1f0,
            ];

pub fn constants_k_sha1(t:usize) -> u32 {
    match t {
        0..20 => 0x5a827999,
        20..40 => 0x6ed9eba1,
        40..60 => 0x8f1bbcdc,
        60..80 => 0xca62c1d6,
        _ => panic!("Impossible number in SHA1 function"),
    }
}
