use bevy::{
    diagnostic::{
        Diagnostic, DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
    },
    prelude::*,
    window::WindowResolution,
};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(17, 17, 17)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#game".to_owned()),
            resolution: WindowResolution::new(900.0, 720.0),
            prevent_default_event_handling: true,
            ..default()
        }),
        ..default()
    }));

    app.add_plugins(DebugPlugin);

    app.add_systems(Startup, setup_camera);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    use bevy::render::camera::ScalingMode;

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(20.0);
    commands.spawn(camera);
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(EntityCountDiagnosticsPlugin)
            .add_systems(Startup, (setup_debug_fps, setup_debug_entity_count))
            .add_systems(Last, (debug_fps_system, debug_entity_count_system));
    }
}

#[derive(Component)]
pub struct DebugFps;

#[derive(Component)]
pub struct DebugEntityCount;

fn setup_debug_fps(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS ",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        }),
        DebugFps,
    ));
}

fn setup_debug_entity_count(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "ENTITIES ",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(40.0),
            left: Val::Px(10.0),
            ..Default::default()
        }),
        DebugEntityCount,
    ));
}

fn debug_fps_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DebugFps>>,
) {
    for mut text in &mut query {
        let fps = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(Diagnostic::smoothed);
        if let Some(fps) = fps {
            text.sections[1].value = format!("{fps:.2}");
        }
    }
}

fn debug_entity_count_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DebugEntityCount>>,
) {
    for mut text in &mut query {
        let count = diagnostics
            .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            .and_then(Diagnostic::value);
        if let Some(count) = count {
            text.sections[1].value = format!("{count}");
        }
    }
}
