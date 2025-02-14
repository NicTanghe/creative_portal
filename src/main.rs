// Add a method for tracking characters. and attaching dialogue.

//! This example illustrates scrolling in Bevy UI.

use accesskit::{Node as Accessible, Role};
use bevy::{
    window::WindowResolution,
    color::palettes::css::{GOLD,GRAY,WHITE,DARK_GREY},
    a11y::AccessibilityNode,
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    winit::WinitSettings,
    diagnostic::FrameTimeDiagnosticsPlugin,
    
};



use creative_hub::prelude::{
    inserted_resources::resources::*,
    components::{
        chunks::Chunks,
        texts::*,
    },
    systems::{
        text::*,
        editor::*,
        settings::*,
    },
};

//ok so this is creating the variable but in a fancy way 
//  let screenplay = Chunks::new(screenplay_path);



fn main() {
    let mut app = App::new();
    app
    .insert_resource(
        ChunkersR {
            screenplay: Chunks::new("assets/screenplays/smalltest.fountain") })
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: 
                    WindowResolution::new(500., 300.)
                        .with_scale_factor_override(1.0),
                ..Default::default()
            }),
            ..Default::default()
        }),
        FrameTimeDiagnosticsPlugin,
        ))
    .insert_resource(WinitSettings::desktop_app())
    .add_systems(Startup, setup)
    .add_systems(Update, (
            //text
            update_scroll_position,
            text_update_system,
            update_scrollbar_position,
            cursor_movement_system,
            //scrollbar_drag_system
            

            //settings
            change_scale_factor,
            ));

    app.run();
}

const FONT_SIZE: f32 = 12.;
const LINE_HEIGHT: f32 = 14.;
const SCROLLBAR_WIDTH: f32 = 5.0;
const SCROLL_HANDLE_HEIGHT: f32 = 30.0;


/// **Setup function to create UI layout**
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    chunkers: Res<ChunkersR>

    ) {
    let primary_font = asset_server.load("fonts/Courier-Bold-a Regular.ttf");
    let screenplay_font = asset_server.load("fonts/Courier Regular.ttf");


    // when we change rendering mode to marketup we need to change the line_indeces function
    // so that the double space newlines are added given an indec aswell
    let line_indices = chunkers.screenplay.line_indices();

    //let content = fs::read_to_string(screenplay_path).expect("Failed to read file");

                                          //
    //let cursor_x = cursor_position.2 as f32 * CHARACTER_WIDTH;
    //let cursor_y = cursor_position.1 as f32 * LINE_HEIGHT;
    // Camera
    commands.spawn((Camera2d, IsDefaultUiCamera));

    // Root UI node
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
                .spawn(
                    Node {
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
                            Name::new("fountain_editor"),
                            ScrollableContainer {
                                //line height seems to be uncorrect for some reason scale factor
                                //really fucks with it 2 also how does the scrolfield get larger
                                //when its smaller/
                                //why divided by 10 td ? pretty sure a line is not 1 px in size
                                content_height:line_indices.len() as f32 * LINE_HEIGHT/12.,
                            },
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
                                    if let Some(text) = chunkers.screenplay.get_line(*chunk_idx, *line_idx) {
                                        let display_text = if text.is_empty() { " " } else { text };
                                    parent.spawn((
                                        Text::new(display_text.to_string()),
                                        TextFont {
                                            font: screenplay_font.clone(),
                                            font_size: 12.0,
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
                            // **Spawn Cursor**
                            parent.spawn((
                                Node {
                                    width: Val::Px(2.0), // Thin vertical line
                                    height: Val::Px(LINE_HEIGHT), // Match line height
                                    left: Val::Px(chunkers.screenplay.get_cursor_x(FONT_SIZE)),
                                    top: Val::Px(chunkers.screenplay.get_cursor_y(LINE_HEIGHT)),
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                BackgroundColor(Color::WHITE.into()), // White color for cursor
                                CursorIndicator, // Custom marker for easy updates
                            ));
                        });

                    // Scrollbar Track
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(SCROLLBAR_WIDTH),
                                height: Val::Percent(100.0),
                                left: Val::Px(20.0),
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            //BackgroundColor(DARK_GREY.into()),
                        ))
                        // ok pretty sure we need to change this into a button to simplify things
                        // Well do that later and now focus on editing ects
                        //
                        // the converstation on this was with the nicolaas tanghe acount.
                        // no shit kakke traag en dan zo tog niet en soms tog wel.
                        .with_children(|parent| {
                            // Scrollbar Handle (Draggable)
                            parent.spawn((
                                Node {
                                    width: Val::Px(SCROLLBAR_WIDTH),
                                    height: Val::Px(SCROLL_HANDLE_HEIGHT),
                                    top: Val::Percent(0.), // Start at the top
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

