// Rotate the given u32 to the left in a cirular way
pub fn rotL(nb:u32, n:u32) -> u32 {
    let n = n & 31;
    nb.rotate_left(n)
}

//Ch(X, Y, Z) = (X ∧ Y ) ⊕ (X ∧ Z),
pub fn ch(x:u32, y:u32, z:u32) -> u32 {
    (x & y) ^ (!x & z)
}

//Maj(X, Y, Z) = (X ∧ Y ) ⊕ (X ∧ Z) ⊕ (Y ∧ Z),
pub fn maj(x:u32, y:u32, z:u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

// converts a 4 bytes array to a u32 big endian ordered
pub fn merge(src:&[u8]) -> u32
{
        let mut res:u32 = 0;
        src.iter()
           .for_each(|byte| res = (res << 8) + (byte.to_be_bytes()[0] as u32));
        res
}

// same as above but the output is smaller
pub fn sha1_arr(data:&[u32]) -> [u8;20]
{
        let mut converted:[u8;20] = [0;20];
        let mut i = 0;

        for four_byte in data {
            converted[i] = (four_byte >> 24) as u8;
            converted[i + 1] = (four_byte >> 16) as u8;
            converted[i + 2] = (four_byte >> 8) as u8;
            converted[i + 3] = *four_byte as u8;
            i += 4;
        }

       converted 
}
// fills a given destination array of fixed size with
// sixteen u32 starting at position i in the source byte array
pub fn chunky(src:&[u8]) -> [u32;16]
{
    src.chunks(4)
    .map(|slice| merge(slice))
    .collect::<Vec<u32>>().try_into().expect("conversion failed")
}

pub fn padding(src:&[u8], messageL:usize) 
-> Vec<[u32;16]>
{
    let mut blocks = vec![];

    let mut copy:[u8;64] = [0;64];
    let datalength = src.len() * 8;

    for i in 0..src.len() {
        copy[i] = src[i];
    }

    let k = src.len() / 4;

    let mut padding_block:[u32; 16] = copy.chunks(4)
                  .map(|slice| merge(slice))
                  .collect::<Vec<u32>>()
                  .try_into().expect("conversion failed");

    // blocks need to be 512 bits at most
    if datalength == 512 && (padding_block[15] & 1) == 1{
        blocks.push(padding_block);
        padding_block = [0; 16];
        padding_block[0] = 1 << 31;
    }
    else if datalength == 0 {
        padding_block[0] = 1 << 31;
    }
    else {
        let tmp = padd_a_one(padding_block[k]);

        // if a bit was added, replace with the new data
        if padding_block[k] != tmp {
            padding_block[k] = tmp;
        }
        // current elt has LSB worth 1, hence we append the 1 on the next
        // out of bound is excluded with previous if
        else {
           padding_block[k + 1] = 1 << 31; 
        }

        // length takes 64 bits, hence we must create a new block
        if datalength >= 448 {
            blocks.push(padding_block);
            padding_block = [0; 16];
        }
    }

    let message_length:u64 = (messageL as u64) * (8 as u64);
    padding_block[14] = (message_length >> 32) as u32;
    padding_block[15] = message_length as u32;
    blocks.push(padding_block);

    blocks
}

// adds a padding 1 after the rightmost filled byte
// returns the number given in param if the insertion took place
fn padd_a_one(nb:u32) -> u32
{
    let mut x = 1;
    let mut shift = 0;
    if nb & x == 1 {
        return nb;
    }
    while shift < 31 && (nb & x) == 0 {
        x <<= 1;
        shift += 1;
    }

    if shift < 31 {
        x >>= 1 + (shift & 7);
    }
    // setting a bit to 1
    // the bit we are interested in
    return nb | x;
}

pub fn sha1f(t:usize, x:u32, y:u32, z:u32) -> u32
{
    match t {
        0..20 => ch(x, y, z),
        20..40 | 60..80 => x ^ y ^ z,
        40..60 => maj(x, y, z),
        _ => panic!("Impossible number in SHA1 function"),
    }
}
