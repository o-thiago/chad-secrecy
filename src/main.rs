pub mod encoder;
mod write;

use clap::{Parser, Subcommand};
use write::WriteArgs;

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
        Command::Write(write_args) => write::write(write_args)?,
        Command::Read(..) => todo!(),
    }

    Ok(())
}
