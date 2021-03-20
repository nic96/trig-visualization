use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::consts::*;
use crate::entity::*;
use crate::utils::{clamp, wrap, SpawnLine};
use std::f32::consts::PI;

pub fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        commands.insert_resource(WinitWebResizing::new());
    }

    let font: Handle<Font> = asset_server.load("fonts/FiraMono-Regular.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                border: Rect::all(Val::Px(2.)),
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(10.),
                    left: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgba_linear(0.5, 0.5, 0.5, 0.5).into()),
            visible: Visible {
                is_transparent: true,
                is_visible: true,
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        border: Rect::all(Val::Px(2.)),
                        padding: Rect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba_linear(0.02, 0.02, 0.02, 0.75).into()),
                    visible: Visible {
                        is_transparent: true,
                        is_visible: true,
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            ..Default::default()
                        },
                        text: Text {
                            value: HELP_TEXT.into(),
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            style: TextStyle {
                                font_size: 18.0,
                                color: Color::rgb_linear(0.7, 0.7, 0.7),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .spawn_line(
            materials.add(Color::rgb_linear(0.3, 0.3, 0.3).into()),
            (Vec2::new(-1000., 0.), Vec2::new(1000., 0.)),
            1.,
        )
        .spawn_line(
            materials.add(Color::rgb_linear(0.3, 0.3, 0.3).into()),
            (Vec2::new(0., -1000.), Vec2::new(0., 1000.)),
            1.,
        )
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                padding: Rect::all(Val::Px(10.)),
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            material: materials.add(Color::rgba_linear(0.1, 0.1, 0.1, 0.5).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text {
                        value: "θ = ".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 18.0,
                            color: Color::rgb_linear(0.7, 0.7, 0.7),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(ThetaText)
                .spawn(TextBundle {
                    text: Text {
                        value: "cos θ = ".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 18.0,
                            color: COS_COLOR,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(CosText)
                .spawn(TextBundle {
                    text: Text {
                        value: "sin θ = ".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 18.0,
                            color: SIN_COLOR,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(SinText)
                .spawn(TextBundle {
                    text: Text {
                        value: "tan θ = ".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 18.0,
                            color: TAN_COLOR,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(TanText)
                .spawn(TextBundle {
                    text: Text {
                        value: "cot θ = ".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 18.0,
                            color: COT_COLOR,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(CotText);
        });
}

pub fn animate_system(
    mut theta: ResMut<Theta>,
    paused: Res<Paused>,
    time: Res<Time>,
    mut text: Query<&mut Text, With<ThetaText>>,
) {
    if paused.0 {
        return;
    };

    for mut txt in text.iter_mut() {
        txt.value = format!("θ = {:.3} = {:.1}°", theta.0, theta.0.to_degrees()).into();
    }

    theta.0 = wrap(theta.0 + time.delta_seconds() * 0.5, 0., 2. * PI);
}

pub fn circle_system(
    commands: &mut Commands,
    radius: Res<Radius>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<Circle>>,
) {
    for entity in query.iter() {
        commands.despawn(entity);
    }

    let circle = shapes::Circle {
        radius: radius.0,
        ..shapes::Circle::default()
    };

    commands
        .spawn(GeometryBuilder::build_as(
            &circle,
            materials.add(Color::rgb_linear(0.5, 0.5, 0.5).into()),
            TessellationMode::Stroke(StrokeOptions::default().with_line_width(2.)),
            Transform::default(),
        ))
        .with(Circle);
}

pub fn radius_line_system(
    commands: &mut Commands,
    query: Query<Entity, With<RadiusLine>>,
    theta: Res<Theta>,
    radius: Res<Radius>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands.despawn(entity);
    }

    // cos = width
    let x = radius.0 * theta.0.cos();
    // sin = height
    let y = radius.0 * theta.0.sin();

    commands
        .spawn_line(
            materials.add(Color::GRAY.into()),
            (Vec2::zero(), Vec2::new(x, y)),
            1.,
        )
        .with(RadiusLine);
}

pub fn cos_line_system(
    commands: &mut Commands,
    query: Query<Entity, With<CosLine>>,
    mut text: Query<&mut Text, With<CosText>>,
    theta: Res<Theta>,
    radius: Res<Radius>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands.despawn(entity);
    }

    // cos = width
    let x = radius.0 * theta.0.cos();
    // sin = height
    let y = radius.0 * theta.0.sin();

    for mut txt in text.iter_mut() {
        txt.value = format!("cos θ = {:.5}", theta.0.cos()).into();
    }

    commands
        .spawn_line(
            materials.add(COS_COLOR.into()),
            (Vec2::new(0., y), Vec2::new(x, y)),
            2.,
        )
        .with(CosLine);
}

pub fn sin_line_system(
    commands: &mut Commands,
    query: Query<Entity, With<SinLine>>,
    mut text: Query<&mut Text, With<SinText>>,
    theta: Res<Theta>,
    radius: Res<Radius>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands.despawn(entity);
    }

    // cos = width
    let x = radius.0 * theta.0.cos();
    // sin = height
    let y = radius.0 * theta.0.sin();

    for mut txt in text.iter_mut() {
        txt.value = format!("sin θ = {:.5}", theta.0.sin()).into();
    }

    commands
        .spawn_line(
            materials.add(SIN_COLOR.into()),
            (Vec2::new(x, 0.), Vec2::new(x, y)),
            2.,
        )
        .with(SinLine);
}

pub fn tan_line_system(
    commands: &mut Commands,
    query: Query<Entity, With<TanLine>>,
    mut text: Query<&mut Text, With<TanText>>,
    theta: Res<Theta>,
    radius: Res<Radius>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands.despawn(entity);
    }

    let x = radius.0 * theta.0.cos();
    let y = radius.0 * theta.0.sin();
    // we use secant to calculate the x coordinate of the end
    // of the tangent line, no tan function is actually used
    // to draw the line. The tan function would be used to get
    // the line's length
    let secant = theta.0.cos().recip();
    let end_x = secant * radius.0;

    for mut txt in text.iter_mut() {
        txt.value = format!("tan θ = {:.5}", theta.0.tan()).into();
    }

    commands
        .spawn_line(
            materials.add(TAN_COLOR.into()),
            (Vec2::new(x, y), Vec2::new(clamp(end_x, -9000., 9000.), 0.)),
            2.,
        )
        .with(SinLine);
}

pub fn cot_line_system(
    commands: &mut Commands,
    query: Query<Entity, With<CotLine>>,
    mut text: Query<&mut Text, With<CotText>>,
    theta: Res<Theta>,
    radius: Res<Radius>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for entity in query.iter() {
        commands.despawn(entity);
    }

    let x = radius.0 * theta.0.cos();
    let y = radius.0 * theta.0.sin();
    // we use cosecant to calculate the y coordinate of the end
    // of the cotangent line, no cot function is actually used
    // to draw the line. The cot function would be used to get
    // the line's length
    let cosecant = theta.0.sin().recip();
    let end_y = cosecant * radius.0;

    for mut txt in text.iter_mut() {
        let cot = theta.0.tan().recip();
        txt.value = format!("cot θ = {:<12}", format!("{:.5}", cot)).into();
    }

    commands
        .spawn_line(
            materials.add(COT_COLOR.into()),
            (Vec2::new(x, y), Vec2::new(0., clamp(end_y, -9000., 9000.))),
            2.,
        )
        .with(SinLine);
}

#[derive(Default)]
pub struct MouseState {
    cursor_pos: Vec2,
}

pub fn mouse_click_system(
    mut state: Local<MouseState>,
    mut theta: ResMut<Theta>,
    paused_button_hovered: Res<PauseButtonHovered>,
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        state.cursor_pos = pos - Vec2::new(window.width() as f32 / 2., window.height() as f32 / 2.);
    }

    if mouse_button_input.pressed(MouseButton::Left) {
        if !paused_button_hovered.0 {
            let angle = state.cursor_pos.y.atan2(state.cursor_pos.x);
            theta.0 = angle;
        }
    }
}

pub fn resize_circle_system(mut radius: ResMut<Radius>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let new_radius = window.width() / 2. - 20.;
    radius.0 = if new_radius > 200. { 200. } else { new_radius };
}

#[cfg(target_arch = "wasm32")]
use bevy::winit::WinitWindows;
#[cfg(target_arch = "wasm32")]
pub fn web_resize_system(winit_windows: Res<WinitWindows>, web_resizing: Res<WinitWebResizing>) {
    use bevy::window::WindowId;
    let winit_window = winit_windows.get_window(WindowId::primary()).unwrap();
    for size in web_resizing.rx.clone().try_iter().last() {
        winit_window.set_inner_size(size);
    }
}
