// Add a method for tracking characters. and attaching dialogue.

//! This example illustrates scrolling in Bevy UI.

use accesskit::{Node as Accessible, Role};
use bevy::{
    color::palettes::css::{GOLD,GRAY,WHITE,DARK_GREY},
    a11y::AccessibilityNode,
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    winit::WinitSettings,
    diagnostic::FrameTimeDiagnosticsPlugin,
};


use std::fs;

use creative_hub::prelude::{
    components::{
        chunks::Chunks,
        texts::*,
    },
    systems::text::*,
}


;fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        FrameTimeDiagnosticsPlugin
        )).insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, (
                update_scroll_position,
                text_update_system,
                //scrollbar_drag_system
                ));

    app.run();
}

const FONT_SIZE: f32 = 12.;
const LINE_HEIGHT: f32 = 21.;
const SCROLLBAR_WIDTH: f32 = 10.0;
const SCROLL_HANDLE_HEIGHT: f32 = 30.0;

/// **Setup function to create UI layout**
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let primary_font = asset_server.load("fonts/Courier-Bold-a Regular.ttf");
    let screenplay_font = asset_server.load("fonts/Courier Regular.ttf");

    let screenplay_path = "assets/screenplays/bigfish.fountain";
    let screenplay = Chunks::new(screenplay_path);
    let line_indices = screenplay.line_indices();

    let content = fs::read_to_string(screenplay_path).expect("Failed to read file");

    // Camera
    commands.spawn((Camera2d, IsDefaultUiCamera));

    // Root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(88.),
                    top: Val::Percent(5.),
                    left: Val::Percent(10.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|parent| {
                    // Scrollable content container
                     parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(100.),
                                overflow: Overflow::scroll_y(), // n.b.
                                ..default()
                            },
                            //BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                        ))
                        .with_children(|parent| {
                            for (chunk_idx, line_idx) in &line_indices {
                                    if let Some(text) = screenplay.get_line(*chunk_idx, *line_idx) {
                                        let display_text = if text.is_empty() { " " } else { text };
                                    parent.spawn((
                                        Text::new(display_text.to_string()),
                                        TextFont {
                                            font: screenplay_font.clone(),
                                            ..default()
                                        },
                                        Label,
                                        AccessibilityNode(Accessible::new(Role::ListItem)),
                                    ))
                                    .insert(PickingBehavior {
                                        should_block_lower: false,
                                        ..default()
                                    });
                                }
                            }
                        });

                    // Scrollbar Track
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(SCROLLBAR_WIDTH),
                                height: Val::Percent(100.0),
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            BackgroundColor(DARK_GREY.into()),
                        ))
                        .with_children(|parent| {
                            // Scrollbar Handle (Draggable)
                            parent.spawn((
                                Node {
                                    width: Val::Px(SCROLLBAR_WIDTH),
                                    height: Val::Px(SCROLL_HANDLE_HEIGHT),
                                    top: Val::Px(0.0), // Start at the top
                                    ..default()
                                },
                                BackgroundColor(WHITE.into()),
                                ScrollbarHandle,
                                //ScrollbarDragging(false),
                            ));
                        });
                });
        });

    // FPS Counter
    commands.spawn((
        Text::new("FPS: "),
        TextFont {
            font: primary_font.clone(),
            font_size: 22.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        TextFont {
            font: screenplay_font.clone(),
            font_size: 18.0,
            ..default()
        },
        TextColor(GOLD.into()),
        FpsText,
    ));
}

