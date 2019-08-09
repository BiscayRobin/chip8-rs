//to make as a library + define more
pub trait GetBits {
	fn get_bit(&self, index: usize) -> bool;
}
