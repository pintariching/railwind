use std::cell::OnceCell;
use std::collections::HashMap;

use macro_derive::GetOnceCell;

mod layout;

#[derive(Default)]
pub struct Config {
    pub backgrounds: BackgroundsConfig,
    pub spacing: SpacingConfig,
    pub borders: BordersConfig,
    pub effects: EffectsConfig,
    pub filters: FiltersConfig,
    pub flexbox_grid: FlexboxGridConfig,
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

#[derive(GetOnceCell, Default)]
pub struct FiltersConfig {
    #[config_path("filters/blur.ron")]
    blur: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/brightness.ron")]
    brightness: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/contrast.ron")]
    contrast: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/drop_shadow.ron")]
    drop_shadow: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/grayscale.ron")]
    grayscale: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/hue_rotate.ron")]
    hue_rotate: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/invert.ron")]
    invert: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/saturate.ron")]
    saturate: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/sepia.ron")]
    sepia: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("filters/opacity.ron")]
    backdrop_opacity: OnceCell<HashMap<&'static str, &'static str>>,
}

#[derive(GetOnceCell, Default)]
pub struct FlexboxGridConfig {
    #[config_path("flexbox_grid/basis.ron")]
    basis: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/flex.ron")]
    flex: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grow.ron")]
    grow: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/shrink.ron")]
    shrink: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/order.ron")]
    order: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_template_columns.ron")]
    grid_template_columns: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_column_span.ron")]
    grid_column_span: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_column_start_end.ron")]
    grid_column_start: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_column_start_end.ron")]
    grid_column_end: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_template_rows.ron")]
    grid_template_rows: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_row_span.ron")]
    grid_row_span: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_row_start_end.ron")]
    grid_row_start: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_row_start_end.ron")]
    grid_row_end: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_auto_columns.ron")]
    grid_auto_columns: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/grid_auto_rows.ron")]
    grid_auto_rows: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/gap.ron")]
    gap: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/gap.ron")]
    gap_x: OnceCell<HashMap<&'static str, &'static str>>,

    #[config_path("flexbox_grid/gap.ron")]
    gap_y: OnceCell<HashMap<&'static str, &'static str>>,
}
