use crate::Error;
use crate::code::LDPCCode; // Import the LDPCCode type

pub trait Decoder{
	fn decode(&self, code: LDPCCode) -> Error;
}

