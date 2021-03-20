use bevy::render::prelude::Color;

pub const COS_COLOR: Color = Color::rgb_linear(0.8, 0.1, 0.1);
pub const SIN_COLOR: Color = Color::rgb_linear(0.1, 0.2, 0.9);
pub const TAN_COLOR: Color = Color::rgb_linear(0.1, 0.6, 0.1);
pub const COT_COLOR: Color = Color::rgb_linear(0.6, 0.6, 0.1);

pub const HELP_TEXT: &str = r#"
It's often helpful to think of cosine as width,
sine as height, and tangent as slope.
"#;
