mod nb_utils;
mod utils;
use crate::utils::sha::utils::*;
use crate::utils::sha::nb_utils::*;

#[derive(Debug, Clone)]
pub struct BlockSHA1 {
    // 64 * 32 bits = 2048 bits
    sub_blocks: [u32; 80],
}

impl BlockSHA1 {
    fn new(data: &[u32;16]) -> BlockSHA1 {
        let mut sub_blocks:[u32;80] = [0;80]; 
        for i in 0..16
        {
            sub_blocks[i] = data[i];
            println!("{:032b}", sub_blocks[i]);
        }
        // 16 <= t < 80 
        // W(t) = S^1(W(t-3) XOR W(t-8) XOR W(t-14) XOR W(t-16)).
        for i in 16..80
        {
            let tmp = sub_blocks[i - 3] ^ sub_blocks[i - 8]
                ^ sub_blocks[i - 14] ^ sub_blocks[i - 16];
            sub_blocks[i] = rotL(tmp, 1);
            println!("{:032b}", sub_blocks[i]);
        }
        
        BlockSHA1 {
            sub_blocks,
        }
    }
    fn process(&self, sha: &mut Sha1)
    {
        let mut vu = sha.variables;
        for i in 0..80 {

            // TEMP = S^5(A) + f(t;B,C,D) + E + W(t) + K(t);
            let temp = (rotL(vu[0], 5) as u64 
                       + sha1f(i, vu[1], vu[2], vu[3]) as u64
                       + vu[4] as u64
                       + self.sub_blocks[i] as u64
                       + constants_k_sha1(i) as u64) as u32;

            for j in (1..5).rev() {
                vu[j] = vu[j - 1];
            }

            vu[2] = rotL(vu[2], 30);
            vu[0] = temp;
        }

        for i in 0..5 {
            sha.variables[i] = ((sha.variables[i] as u64) + (vu[i] as u64)) as u32;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sha1 {
    variables: [u32;5], // fixed size of 8 : [a,b,c,d,e] in the paper
    pub size: usize, // the number of blocks processed
    leftover: Vec<u8>,
}

impl Sha1 {
    pub fn new() -> Sha1
    {
        let size = 0;
        let variables = sha1_init.clone();
        let leftover = vec![];

        Sha1 {
            variables,
            size,
            leftover,
        }
    }

    pub fn update(&mut self, data:&[u8])
    {
        self.size += data.len();
        self.leftover.append(&mut Vec::from(data));

        let bind = self.leftover.clone();

        let mut iter = bind.chunks(64);
        self.leftover = iter.next_back().unwrap_or_else(|| &[]).to_vec();

        iter.for_each(|slice| BlockSHA1::new(&chunky(slice)).process(self));
    }

    pub fn digest(&mut self) {
        padding(&self.leftover, self.size).iter()
            .for_each(|raw_block| BlockSHA1::new(raw_block).process(self));
    }

    pub fn clear(&mut self) {
            self.size = 0;
            self.variables = sha1_init.clone();
            self.leftover = vec![];
    }

    pub fn digest_string(&self) -> String {
        self.variables.iter().map(|x| format!("{:08x}", x)).collect::<String>()
    }
    
    pub fn digest_raw(&self) -> String {
        sha1_arr(&self.variables).iter().map(|x| char::from(*x)).collect::<String>()
    }
    //char::from_u32(x.clone()).unwrap()
}
