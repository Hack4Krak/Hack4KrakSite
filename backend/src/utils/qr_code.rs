use crate::utils::error::Error;
use image::{DynamicImage, ImageFormat, Luma};
use qrcode::QrCode;
use std::io::Cursor;

pub fn generate_qr_png(code: &str) -> Result<Vec<u8>, Error> {
    let qr = QrCode::new(code.as_bytes())
        .map_err(|err| Error::FailedToGenerateQrCode(err.to_string()))?;

    let image = qr.render::<Luma<u8>>().min_dimensions(200, 200).build();

    let mut png_bytes = Vec::new();
    DynamicImage::ImageLuma8(image)
        .write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)
        .map_err(|err| Error::FailedToGenerateQrCode(err.to_string()))?;

    Ok(png_bytes)
}
