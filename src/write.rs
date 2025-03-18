use rand_pcg::Pcg64;
use rand_seeder::Seeder;

use crate::{AMOUNT_RGB_CHANNELS, BYTE_LEN, Error, encoder};

#[derive(Debug, clap::Args)]
pub struct WriteArgs {
    /// Path to the image that will be stored with a secret.
    #[arg(short, long)]
    image_path: std::path::PathBuf,

    /// The message to be encoded on the image.
    #[arg(short, long)]
    message: String,

    /// The seed to encode the image with.
    #[arg(short, long)]
    seed: Option<String>,
}

pub fn write(
    WriteArgs {
        image_path,
        message,
        seed,
        ..
    }: WriteArgs,
) -> Result<(), Error> {
    let dynamic_image = image::open(image_path)
        .map_err(|_| anyhow::anyhow!("Failed to open the requested image"))?;

    let pixels_count = dynamic_image.width() * dynamic_image.height();
    let amount_of_bits_to_encode = message.len() * BYTE_LEN;

    // We encode the bits on the least significant bit of each of the three rgb channels.
    // This will be improved further, so it uses more bits on each channel, rather
    // than throwing an error when the message is too large.
    let amount_of_encoded_pixels = amount_of_bits_to_encode.div_ceil(AMOUNT_RGB_CHANNELS);

    if amount_of_encoded_pixels > usize::try_from(pixels_count).map_err(anyhow::Error::from)? {
        Err(anyhow::anyhow!(
            "Message too large to be encoded on a ${}x${} image",
            dynamic_image.width(),
            dynamic_image.height()
        ))?;
    }

    let mut img_buffer = dynamic_image.to_rgba32f();
    let mut img_pixels: Vec<_> = img_buffer.pixels_mut().collect();

    let mut rng: Pcg64 = Seeder::from(seed.unwrap_or_default()).into_rng();
    encoder::Encoder::new(&mut img_pixels, message.as_bytes())
        .encode_to_image(&mut rng, amount_of_encoded_pixels);

    Ok(())
}
