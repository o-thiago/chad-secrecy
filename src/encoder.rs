use crate::BYTE_LEN;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct EncodingPosition(pub usize, pub usize);

pub struct Encoder {}

impl Encoder {
    pub fn get_encoding_position(
        &self,
        flatten_initial_bit_index: usize,
        rgb_channel_index: usize,
    ) -> EncodingPosition {
        EncodingPosition(
            (flatten_initial_bit_index + 1) / BYTE_LEN,
            (flatten_initial_bit_index % BYTE_LEN) - rgb_channel_index,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bit_positions_are_correct() {
        let encoder = Encoder {};

        assert_eq!(encoder.get_encoding_position(0, 0), EncodingPosition(0, 0));
        assert_eq!(encoder.get_encoding_position(1, 0), EncodingPosition(0, 1));
        assert_eq!(encoder.get_encoding_position(8, 0), EncodingPosition(1, 0));
        assert_eq!(encoder.get_encoding_position(9, 0), EncodingPosition(1, 1));
    }
}
