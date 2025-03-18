use image::Rgba;
use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Debug, clap::Args)]
pub struct ReadArgs {
    /// Path to the image that contains the secret message.
    #[arg(short, long)]
    image_path: std::path::PathBuf,

    /// The seed to read the image from.
    seed: Option<String>,
}

pub struct Decoder<'a> {
    pub pixels: &'a mut Vec<&'a mut Rgba<f32>>,
}

impl<'a> Decoder<'a> {
    pub fn new(pixels: &'a mut Vec<&'a mut Rgba<f32>>) -> Self {
        Self { pixels }
    }

    pub fn decode<R: Rng + Sized>(rgn: &mut R) {}
}

pub fn read(
    ReadArgs {
        image_path, seed, ..
    }: ReadArgs,
) -> Result<(), crate::Error> {
    let dynamic_image = image::open(image_path).map_err(|_| crate::Error::ImageOpenFailed)?;

    let mut img_buffer = dynamic_image.to_rgba32f();
    let mut img_pixels: Vec<_> = img_buffer.pixels_mut().collect();

    let mut rng: Pcg64 = Seeder::from(seed.unwrap_or_default()).into_rng();

    Ok(())
}
