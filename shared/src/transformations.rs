use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Transformation {
    Crop {
        region: Region,
    },
    Grayscale {
        region: Option<Region>,
    },
    Rotate90,
    Rotate180,
    Rotate270,
    FlipVertical {
        region: Option<Region>,
    },
    FlipHorizontal {
        region: Option<Region>,
    },
    Brighten {
        value: i32,
        region: Option<Region>,
    },
    Contrast {
        contrast: f32,
        region: Option<Region>,
    },
    Blur {
        sigma: f32,
        region: Option<Region>,
    },
    Resize {
        width: u32,
        height: u32,
    },
}

impl Transformation {
    pub fn crop(region: Region) -> Self {
        Self::Crop { region }
    }

    pub fn grayscale() -> Self {
        Self::Grayscale { region: None }
    }

    pub fn rotate90() -> Self {
        Self::Rotate90
    }

    pub fn rotate180() -> Self {
        Self::Rotate180
    }

    pub fn rotate270() -> Self {
        Self::Rotate270
    }

    pub fn flip_horizontal() -> Self {
        Self::FlipHorizontal { region: None }
    }

    pub fn flip_vertical() -> Self {
        Self::FlipVertical { region: None }
    }

    pub fn brighten(value: i32) -> Self {
        Self::Brighten {
            value,
            region: None,
        }
    }

    pub fn contrast(contrast: f32) -> Self {
        Self::Contrast {
            contrast,
            region: None,
        }
    }

    pub fn blur(sigma: f32) -> Self {
        Self::Blur {
            sigma,
            region: None,
        }
    }

    pub fn resize(width: u32, height: u32) -> Self {
        Self::Resize { width, height }
    }

    pub fn with_region(mut self, region: Region) -> Self {
        match &mut self {
            Transformation::Grayscale { region: r }
            | Transformation::FlipHorizontal { region: r }
            | Transformation::FlipVertical { region: r }
            | Transformation::Brighten { region: r, .. }
            | Transformation::Contrast { region: r, .. }
            | Transformation::Blur { region: r, .. } => {
                *r = Some(region);
            }
            _ => {
                panic!("This transformation does not support regions");
            }
        }
        self
    }
}
