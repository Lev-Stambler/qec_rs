use code::LDPCCode;
use traits::{Decoder};

pub(crate) type Error = Vec<usize>;

mod traits;
mod weight_reduction;
mod code;

struct BPDecoder {
    max_iter: u32,
    // damping: f64,
}

impl Decoder for BPDecoder {
    fn decode(&self, code: LDPCCode) -> Error {
        // Here we follow https://radfordneal.github.io/LDPC-codes/decoding.html#decode
        // let pchk_file = code.get_alist_path();
        // // TODO: random string here
        // let decoded_file = "/tmp/decoded";
        // let channel = "bsc";
        // let method
        // let decode_cmd = format!(
        //     "././LDPC-2012-02-11/decode {} {} {} {} {} {}",
        //     pchk_file, recieved_file, decoded_file, bp_file, channel, method
        // );
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
