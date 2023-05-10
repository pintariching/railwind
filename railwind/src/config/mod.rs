use std::cell::OnceCell;
use std::collections::HashMap;

use macro_derive::GetOnceCell;

#[derive(Default)]
pub struct Config {
    pub backgrounds: BackgroundsConfig,
    pub spacing: SpacingConfig,
    pub borders: BordersConfig,
    pub effects: EffectsConfig,
}

#[derive(GetOnceCell, Default)]
pub struct BackgroundsConfig {
    #[config_path("colors.ron")]
    color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("backgrounds/background_position.ron")]
    position: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("backgrounds/background_size.ron")]
    size: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("backgrounds/background_image.ron")]
    image: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    gradient_color_stops: OnceCell<HashMap<&'static str, &'static str>>,
}

#[derive(GetOnceCell, Default)]
pub struct SpacingConfig {
    #[config_path("spacing/padding.ron")]
    padding: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("spacing/margin.ron")]
    margin: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("spacing/space_between.ron")]
    space_between: OnceCell<HashMap<&'static str, &'static str>>,
}

#[derive(GetOnceCell, Default)]
pub struct BordersConfig {
    #[config_path("borders/border_radius.ron")]
    border_radius: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("borders/border_width.ron")]
    border_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    border_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("borders/divide_width.ron")]
    divide_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    divide_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("borders/outline_width.ron")]
    outline_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    outline_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("borders/border_width.ron")]
    outline_offset: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("borders/ring_width.ron")]
    ring_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    ring_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("borders/outline_width.ron")]
    ring_offset_width: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    ring_offset_color: OnceCell<HashMap<&'static str, &'static str>>,
}

#[derive(GetOnceCell, Default)]
pub struct EffectsConfig {
    #[config_path("effects/box_shadow.ron")]
    box_shadow: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("colors.ron")]
    box_shadow_color: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("effects/opacity.ron")]
    opacity: OnceCell<HashMap<&'static str, &'static str>>,
}
