mod read;
mod write;

use clap::{Parser, Subcommand};
use read::{ReadArgs, read};
use write::{WriteArgs, write};

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

    #[error("Failed to open the requested image")]
    ImageOpenFailed,

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

const AMOUNT_RGB_CHANNELS: usize = 3;
const BYTE_LEN: usize = 8;

fn main() -> Result<(), Error> {
    let Args { command, .. } = Args::parse();

    (match command {
        Command::Write(write_args) => write(write_args),
        Command::Read(read_args) => read(read_args),
    })?;

    Ok(())
}
