use anyhow::anyhow;
use clap::Parser;
use image::Rgba;

/// Hide secrets in plain sight.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the image that will be stored with a secret.
    #[arg(short)]
    image_path: std::path::PathBuf,

    /// The message to be encoded on the image
    #[arg(short, long)]
    message: String,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("An unexpected state happened when executing the program")]
    Unknown,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

const AMOUNT_RGB_CHANNELS: usize = 3;
const BYTE_LEN: usize = 8;

fn main() -> Result<(), Error> {
    let Args {
        message,
        image_path,
        ..
    } = Args::parse();

    let dynamic_image =
        image::open(image_path).map_err(|_| anyhow!("Failed to open the requested image"))?;

    let pixels_count = dynamic_image.width() * dynamic_image.height();
    let amount_of_bits_to_encode = message.len() * BYTE_LEN;

    // We encode the bits on the least significant bit of each of the three rgb channels.
    // This will be improved further, so it uses more bits on each channel, rather
    // than throwing an error when the message is too large.
    let amount_of_encoded_pixels = amount_of_bits_to_encode.div_ceil(AMOUNT_RGB_CHANNELS);

    if amount_of_encoded_pixels > usize::try_from(pixels_count).map_err(anyhow::Error::from)? {
        Err(anyhow!(
            "Message too large to be encoded on  a ${}x${} image",
            dynamic_image.width(),
            dynamic_image.height()
        ))?;
    }

    let mut img_buffer = dynamic_image.to_rgba32f();
    let mut img_pixels: Vec<_> = img_buffer.pixels_mut().collect();

    let message_bytes = message.as_bytes();

    for (i, pixel) in img_pixels
        .iter_mut()
        .take(amount_of_encoded_pixels)
        .enumerate()
    {
        let flatten_initial_bit_index = (i * AMOUNT_RGB_CHANNELS).max(0);
        for (i, rgb_value) in pixel.0.iter_mut().take(AMOUNT_RGB_CHANNELS).enumerate() {
            let current_byte = message_bytes[flatten_initial_bit_index / BYTE_LEN];
            let read_bit =
                (1 << (flatten_initial_bit_index % BYTE_LEN) - i) & usize::from(current_byte);
            *rgb_value = 0.;
        }
    }

    Ok(())
}
