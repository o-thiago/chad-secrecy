pub mod encoder;

use anyhow::anyhow;
use clap::Parser;

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
    _Unknown,

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
            "Message too large to be encoded on a ${}x${} image",
            dynamic_image.width(),
            dynamic_image.height()
        ))?;
    }

    let mut img_buffer = dynamic_image.to_rgba32f();
    let mut img_pixels: Vec<_> = img_buffer.pixels_mut().collect();

    let message_bytes = message.as_bytes();
    let mut encoder = encoder::Encoder::new(&mut img_pixels, message_bytes);
    encoder.encode_to_image(amount_of_encoded_pixels);

    Ok(())
}
