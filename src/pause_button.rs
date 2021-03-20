use crate::entity::{PauseButtonHovered, Paused};
use bevy::prelude::*;

pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            hovered: materials.add(Color::rgb(0.35, 0.35, 0.35).into()),
            pressed: materials.add(Color::rgb(0.45, 0.45, 0.45).into()),
        }
    }
}

pub fn pause_button_system(
    mut paused: ResMut<Paused>,
    mut pause_button_hovered: ResMut<PauseButtonHovered>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Mutated<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                paused.0 = !paused.0;
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
                pause_button_hovered.0 = true;
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
                pause_button_hovered.0 = false;
            }
        }
        if paused.0 {
            text.value = "Resume".into();
        } else {
            text.value = "Pause".into();
        }
    }
}

pub fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        // ui camera
        .spawn(CameraUiBundle::default())
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(42.0)),
                // center button
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: "Pause".into(),
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    style: TextStyle {
                        font_size: 24.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
        });
}
