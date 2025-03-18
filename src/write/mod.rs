pub mod encoder;
pub mod write;

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
