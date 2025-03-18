pub mod encoder;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Debug, Clone, Copy, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
    Write,
    Read,
}

/// Hide secrets in plain sight.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Args)]
struct WriteArgs {
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

#[derive(Debug, clap::Args)]
struct ReadArgs {
    /// Path to the image that contains the secret message.
    #[arg(short, long)]
    image_path: std::path::PathBuf,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Write mode: encode a message into an image.
    Write(WriteArgs),
    /// Read mode: decode a message from an image.
    Read(ReadArgs),
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
    let Args { command, .. } = Args::parse();

    match command {
        Command::Write(write_args) => write(write_args)?,
        Command::Read(..) => todo!(),
    }

    Ok(())
}

fn write(
    WriteArgs {
        image_path,
        message,
        seed,
        ..
    }: WriteArgs,
) -> Result<(), Error> {
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

    let mut rng: Pcg64 = Seeder::from(seed.unwrap_or_default()).into_rng();
    encoder::Encoder::new(&mut img_pixels, message.as_bytes())
        .encode_to_image(&mut rng, amount_of_encoded_pixels);

    Ok(())
}
