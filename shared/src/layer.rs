use crate::{Region, Transformation};
use image::{DynamicImage, ImageFormat, RgbaImage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Layer {
    #[serde(skip)]
    pub image: DynamicImage,
    #[serde(skip)]
    image_format: Option<ImageFormat>,
    #[serde(with = "serde_bytes")]
    image_data: Vec<u8>,
}

impl Layer {
    pub fn new(image_data: &[u8]) -> Result<Layer, String> {
        let image = image::load_from_memory(image_data)
            .map_err(|e| format!("Failed to load image: {}", e))?;
        let image_format = image::guess_format(image_data).ok();
        Ok(Layer {
            image,
            image_format,
            image_data: image_data.to_vec(),
        })
    }

    pub fn new_empty(width: u32, height: u32) -> Result<Self, String> {
        let image = DynamicImage::ImageRgba8(RgbaImage::new(width, height));
        let mut bytes = Vec::new();
        image
            .write_to(&mut std::io::Cursor::new(&mut bytes), ImageFormat::Png)
            .map_err(|e| format!("Failed to encode empty image: {}", e))?;

        Ok(Layer {
            image,
            image_format: Some(ImageFormat::Png),
            image_data: bytes,
        })
    }

    fn apply_region_transformation(
        &mut self,
        region: &Region,
        transform: Box<dyn Fn(&mut DynamicImage)>,
    ) -> Result<(), String> {
        if region.x >= self.image.width()
            || region.y >= self.image.height()
            || region.x + region.width > self.image.width()
            || region.y + region.height > self.image.height()
        {
            return Err("Region is out of bounds".to_string());
        }

        let mut sub_image = self
            .image
            .crop_imm(region.x, region.y, region.width, region.height);

        transform(&mut sub_image);

        // Convert both images to RGBA8 for pixel manipulation
        let mut new_image = self.image.to_rgba8();
        let sub_rgba = sub_image.to_rgba8();

        // Copy the transformed region back
        for y in 0..region.height {
            for x in 0..region.width {
                let pixel = sub_rgba.get_pixel(x, y);
                new_image.put_pixel(x + region.x, y + region.y, *pixel);
            }
        }

        self.image = DynamicImage::ImageRgba8(new_image);
        Ok(())
    }

    fn apply_transform_with_region(
        &mut self,
        region: Option<Region>,
        transform: Box<dyn Fn(&DynamicImage) -> DynamicImage>,
    ) -> Result<(), String> {
        match region {
            Some(region) => self.apply_region_transformation(
                &region,
                Box::new(move |img| {
                    *img = transform(img);
                }),
            ),
            None => {
                self.image = transform(&self.image);
                Ok(())
            }
        }
    }

    pub fn apply_transformation(&mut self, transformation: Transformation) -> Result<(), String> {
        match transformation {
            Transformation::Crop { region } => {
                self.image = self
                    .image
                    .crop_imm(region.x, region.y, region.width, region.height);
                Ok(())
            }
            Transformation::Grayscale { region } => {
                self.apply_transform_with_region(region, Box::new(|img| img.grayscale()))
            }
            Transformation::FlipHorizontal { region } => {
                self.apply_transform_with_region(region, Box::new(|img| img.fliph()))
            }
            Transformation::FlipVertical { region } => {
                self.apply_transform_with_region(region, Box::new(|img| img.flipv()))
            }
            Transformation::Rotate90 => {
                self.image = self.image.rotate90();
                Ok(())
            }
            Transformation::Rotate180 => {
                self.image = self.image.rotate180();
                Ok(())
            }
            Transformation::Rotate270 => {
                self.image = self.image.rotate270();
                Ok(())
            }
            Transformation::Brighten { value, region } => {
                self.apply_transform_with_region(region, Box::new(move |img| img.brighten(value)))
            }
            Transformation::Contrast { contrast, region } => self.apply_transform_with_region(
                region,
                Box::new(move |img| img.adjust_contrast(contrast)),
            ),
            Transformation::Blur { sigma, region } => {
                self.apply_transform_with_region(region, Box::new(move |img| img.blur(sigma)))
            }
            Transformation::Resize { width, height } => {
                self.image =
                    self.image
                        .resize(width, height, image::imageops::FilterType::Triangle);
                Ok(())
            }
            Transformation::Resize2 { width, height } => {
                self.image = self
                    .image
                    .resize(width, height, image::imageops::FilterType::Nearest);
                Ok(())
            }
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut bytes: Vec<u8> = Vec::new();

        self.image
            .write_to(
                &mut std::io::Cursor::new(&mut bytes),
                self.image_format.unwrap_or(ImageFormat::Png),
            )
            .map_err(|e| format!("Failed to encode image: {}", e))?;
        Ok(bytes)
    }
}
