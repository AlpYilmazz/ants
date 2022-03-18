use std::{fs::File, io::BufReader};

use bevy::prelude::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct GlobalSettings {
    spawn_ants: u32,
}

struct AntsHome {
    pub location: Transform
}

struct TimeScale(f32);

#[derive(Component)]
struct Ant;

#[derive(Component)]
struct Velocity {
    translation: Vec3
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AntsHome { location: Transform::from_xyz(0.0, 0.0, 1.0) })
        .insert_resource(TimeScale(1.0))
        .add_startup_system_to_stage(StartupStage::PreStartup, import_settings)
        .add_startup_system(setup_environment)
        .add_startup_system(set_edges)
        .add_system(change_time_scale)
        .add_system(randomize_ant_velocity)
        .add_system(move_ants)
        .run();
}

fn set_edges(mut commands: Commands, window: Res<Windows>) {
    let arena_span = 0.9f32;

    let win = window.get_primary().unwrap();
    let (ah, aw) = ( arena_span * win.height(), arena_span * win.width() );

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.4),
            custom_size: Some(Vec2::new(aw, ah)),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn import_settings(mut commands: Commands) {
    let settings_file = BufReader::new(File::open("./config/settings.global.json").unwrap());
    let global_settings: GlobalSettings = serde_json::from_reader(settings_file).unwrap();

    commands.insert_resource(global_settings);
}

fn setup_environment(
    mut commands: Commands,
    global_settings: Res<GlobalSettings>,
    ants_home: Res<AntsHome>
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..Default::default()
            /*flip_x: todo!(),
            flip_y: todo!(),*/
        },
        transform: ants_home.location.clone(),
        ..Default::default()
        /*global_transform: todo!(),
        texture: todo!(),
        visibility: todo!(),*/
    });

    let ants_count = global_settings.spawn_ants;
    for _ in 0..ants_count {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(3.0, 3.0)),
                ..Default::default()
                /*flip_x: todo!(),
                flip_y: todo!(),*/
            },
            transform: ants_home.location.clone(),
            ..Default::default()
            /*global_transform: todo!(),
            texture: todo!(),
            visibility: todo!(),*/
        })
        .insert(Velocity { translation: random_vec3_xy(2.0) })
        .insert(Ant);
    }
}

fn change_time_scale(mut time_scale: ResMut<TimeScale>, key_input: Res<Input<KeyCode>>) {
    if key_input.pressed(KeyCode::Up) {
        time_scale.0 = f32::min(3.0, time_scale.0 + 1.0);
    }
    if key_input.pressed(KeyCode::Down) {
        time_scale.0 = f32::max(1.0, time_scale.0 - 1.0);
    }
}

#[inline]
fn random_vec3_xy(scale: f32) -> Vec3 {
    Vec3::new((random::<f32>() - 0.5) * scale, (random::<f32>() - 0.5) * scale, 0.0)
}

fn randomize_ant_velocity(mut ants_query: Query<&mut Velocity, With<Ant>>) {
    for mut v in ants_query.iter_mut() {
        v.translation += random_vec3_xy(1.0);
    }
}

fn move_ants(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut ants_query: Query<(&Velocity, &mut Transform), With<Ant>>
) {
    let delta_sec = time.delta_seconds();

    for (v, mut pos) in ants_query.iter_mut() {
        pos.translation += v.translation * delta_sec * time_scale.0;
    }
}
