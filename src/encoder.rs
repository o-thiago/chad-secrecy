use crate::BYTE_LEN;

pub struct Encoder {}

impl Encoder {
    fn get_encoding_position(
        flatten_initial_bit_index: i32,
        rgb_channel_index: usize,
    ) -> (usize, usize) {
        (
            (flatten_initial_bit_index + 1) / BYTE_LEN,
            (1 << ((flatten_initial_bit_index % BYTE_LEN) - rgb_channel_index)),
        )
    }
}
