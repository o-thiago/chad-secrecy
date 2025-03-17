use image::Rgba;

use crate::{AMOUNT_RGB_CHANNELS, BYTE_LEN};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct EncodingPosition {
    pub byte_position: usize,
    pub bit_position: usize,
}

impl EncodingPosition {
    pub fn new(byte_position: usize, bit_position: usize) -> Self {
        Self {
            byte_position,
            bit_position,
        }
    }
}

pub struct Encoder<'a> {
    pub pixels: &'a mut Vec<&'a mut Rgba<f32>>,
    pub message_bytes: &'a [u8],
}

pub struct EncoderUtil();

impl EncoderUtil {
    pub fn get_encoding_position(
        &self,
        flatten_initial_bit_index: usize,
        rgb_channel_index: usize,
    ) -> EncodingPosition {
        EncodingPosition {
            byte_position: (flatten_initial_bit_index + 1) / BYTE_LEN,
            bit_position: (flatten_initial_bit_index % BYTE_LEN) - rgb_channel_index,
        }
    }
}

impl<'a> Encoder<'a> {
    pub fn new(pixels: &'a mut Vec<&'a mut Rgba<f32>>, message_bytes: &'a [u8]) -> Self {
        Self {
            pixels,
            message_bytes,
        }
    }

    pub fn encode_to_image(&mut self, amount_of_encoded_pixels: usize) {
        for (i, pixel) in self
            .pixels
            .iter_mut()
            .take(amount_of_encoded_pixels)
            .enumerate()
        {
            let flatten_initial_bit_index = (i * AMOUNT_RGB_CHANNELS).max(0);
            for (i, rgb_value) in pixel.0.iter_mut().take(AMOUNT_RGB_CHANNELS).enumerate() {
                let EncodingPosition {
                    byte_position,
                    bit_position,
                } = EncoderUtil().get_encoding_position(flatten_initial_bit_index, i);

                let current_byte = self.message_bytes[byte_position];
                let read_bit = (1 << bit_position) & usize::from(current_byte);

                *rgb_value = read_bit as f32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoder_encodes_to_correct_position() {
        let encoder_util = EncoderUtil();
        for ((flatten_bit_index, rgb_channel_index), (target_byte_position, target_bit_position)) in [
            ((0, 0), (0, 0)),
            ((1, 0), (0, 1)),
            ((8, 0), (1, 0)),
            ((9, 0), (1, 1)),
        ] {
            assert_eq!(
                encoder_util.get_encoding_position(flatten_bit_index, rgb_channel_index),
                EncodingPosition {
                    byte_position: target_byte_position,
                    bit_position: target_bit_position
                }
            );
        }
    }
}
