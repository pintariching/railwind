use std::cell::OnceCell;
use std::collections::HashMap;

use macro_derive::GetOnceCell;

pub struct Config {
    pub backgrounds: BackgroundsConfig,
    pub spacing: SpacingConfig,
    pub borders: BordersConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            backgrounds: BackgroundsConfig::new(),
            spacing: SpacingConfig::new(),
            borders: BordersConfig::new(),
        }
    }
}

#[derive(GetOnceCell)]
pub struct BackgroundsConfig {
    #[config_path("colors.ron")]
    color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("background_position.ron")]
    position: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("background_size.ron")]
    size: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("background_image.ron")]
    image: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    gradient_color_stops: OnceCell<HashMap<&'static str, &'static str>>,
}

impl BackgroundsConfig {
    pub fn new() -> Self {
        Self {
            color: OnceCell::new(),
            position: OnceCell::new(),
            size: OnceCell::new(),
            image: OnceCell::new(),
            gradient_color_stops: OnceCell::new(),
        }
    }
}

#[derive(GetOnceCell)]
pub struct SpacingConfig {
    #[config_path("padding.ron")]
    padding: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("margin.ron")]
    margin: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("space_between.ron")]
    space_between: OnceCell<HashMap<&'static str, &'static str>>,
}

impl SpacingConfig {
    pub fn new() -> Self {
        Self {
            padding: OnceCell::new(),
            margin: OnceCell::new(),
            space_between: OnceCell::new(),
        }
    }
}

#[derive(GetOnceCell)]
pub struct BordersConfig {
    #[config_path("border_radius.ron")]
    border_radius: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("border_width.ron")]
    border_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    border_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("divide_width.ron")]
    divide_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    divide_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("outline_width.ron")]
    outline_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    outline_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("border_width.ron")]
    outline_offset: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("ring_width.ron")]
    ring_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    ring_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("outline_width.ron")]
    ring_offset_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    ring_offset_color: OnceCell<HashMap<&'static str, &'static str>>,
}

impl BordersConfig {
    pub fn new() -> Self {
        Self {
            border_radius: OnceCell::new(),
            border_width: OnceCell::new(),
            border_color: OnceCell::new(),
            divide_width: OnceCell::new(),
            divide_color: OnceCell::new(),
            outline_width: OnceCell::new(),
            outline_color: OnceCell::new(),
            outline_offset: OnceCell::new(),
            ring_width: OnceCell::new(),
            ring_color: OnceCell::new(),
            ring_offset_width: OnceCell::new(),
            ring_offset_color: OnceCell::new(),
        }
    }
}
