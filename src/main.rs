use bevy::{input::keyboard, prelude::*};
use bevy::app::CoreStage::Update;
use bevy::app::StartupStage::PostStartup;
use bevy::ecs::system::EntityCommands;
use bevy::render::render_resource::Texture;
use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

#[derive(Component)]
struct Position(i32, i32);
#[derive(Component)]
struct PlayerOne;
#[derive(Component)]
struct PlayerTwo;

#[derive(Component)]
struct Ball;

#[derive(Resource)]
struct Score {
    player1: i32,
    player2: i32,
}

#[derive(Resource)]
struct Screen {
    top: f32,
    bottom: f32,
    right: f32,
    left: f32
}

#[derive(Component)]
struct BallDirection {
    direction: Vec2,
    velocity: f32,
}

#[derive(Component)]
struct TitleComponent;

#[derive(Component)]
struct PlayerOneScoreComponent;

#[derive(Component)]
struct PlayerTwoScoreComponent;

#[derive(Component)]
struct PlayerOneWall;

#[derive(Component)]
struct PlayerTwoWall;

#[derive(Component)]
struct BallInfos {
    speed: f32,
    direction: Vec3,
    in_collision: bool
}

impl Default for BallInfos {
    fn default() -> BallInfos {
        BallInfos {
            speed: 0.,
            direction: Vec3::X,
            in_collision: false
        }
    }
}

enum BallGoalHit {
    PlayerOneGoal,
    PlayerTwoGoal,
    None
}

enum PlayerEnum {
    PlayerOne,
    PlayerTwo
}

fn insert_resources(mut commands: &mut Commands) {
    commands.insert_resource(Score {
        player1: 0,
        player2: 0,
    });

    commands.insert_resource( Screen {
        top: 200.,
        bottom: -200.,
        left: -500.,
        right: 500.,
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    insert_resources(&mut commands);

    // camera
    commands.spawn(Camera2dBundle::default());

    // player one
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("Pong.png"),
                transform: Transform::from_xyz(-250., 0., 0.),
                ..default()
            },
            PlayerOne,
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(255., 255., 255., 0.),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                ..default()
            });
        });

    // player two
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("Pong.png"),
                transform: Transform::from_xyz(250., 0., 0.),
                ..default()
            },
            PlayerTwo,
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.25, 0.25, 0.75, 0.),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                ..default()
            });
        });

    // ball
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("Balle.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            Ball,
        ))
        .insert(BallInfos {
            speed: 150.,
            direction: Vec3 { x: 1., y: 1., ..default() },
            .. default()
        })
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.25, 0.25, 0.75, 0.),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                ..default()
            });
        });

    // ** UI ** //

    commands
        .spawn(
            (NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(25.)),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::NoWrap,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.15, 0.15, 0.15, 0.)),
                ..default()
            }),
        )
        .with_children(|global_container| {
            // player one score
            global_container
                .spawn(NodeBundle { ..default() })
                .with_children(|player_one_container| {
                    player_one_container.spawn((
                        TextBundle::from_section(
                            0.to_string(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        ),
                        PlayerOneScoreComponent,
                    ));
                });

            // score title
            global_container
                .spawn(NodeBundle { ..default() })
                .with_children(|score_title_container| {
                    score_title_container.spawn((
                        TextBundle::from_section(
                            "Score",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            flex_grow: 2.,
                            ..default()
                        }),
                        TitleComponent,
                    ));
                });

            // player two score
            global_container
                .spawn(NodeBundle { ..default() })
                .with_children(|player_two_score_container| {
                    player_two_score_container.spawn((
                        TextBundle::from_section(
                            0.to_string(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        ),
                        PlayerTwoScoreComponent,
                    ));
                });
        });
}

fn move_ball(
    mut q: Query<(&mut Transform, &BallInfos), With<Ball>>,
    time: Res<Time>
) {
    let (mut ball_transform, ball_infos) = q.single_mut();
    ball_transform.translation += (ball_infos.direction * ball_infos.speed) * time.delta_seconds();
}

fn collision_ball_top_down(
    mut q: Query<(&Transform, &mut BallInfos)>,
    screenLimit: Res<Screen>
) {
    let (ball_transform, mut ball_infos) = q.single_mut();
    if ball_transform.translation.y > screenLimit.top {
        ball_infos.direction.y = -ball_infos.direction.y;
    }
    if ball_transform.translation.y < screenLimit.bottom {
        ball_infos.direction.y = -ball_infos.direction.y;
    }
}

fn player_collision(
    mut ball_q: Query<(&Transform, &mut BallInfos, &Handle<Image>)>,
    assets: Res<Assets<Image>>,
    player_one_q: Query<(&Transform, &Handle<Image>), With<PlayerOne>>,
    player_two_q: Query<&Transform, With<PlayerTwo>>,
) {
    let (ball_transform, mut ball_info, ball_texture) = ball_q.single_mut();
    let (playerone_transform, playerone_texture) = player_one_q.single();
    let playertwo_transform = player_two_q.single();

    if assets.get(playerone_texture).is_none() { return; }
    if assets.get(ball_texture).is_none() { return; }

    let player_sprite_size = assets.get(playerone_texture).unwrap().texture_descriptor.size;
    let ball_sprite_size = assets.get(ball_texture).unwrap().texture_descriptor.size;


    let player_epsilon_x = (player_sprite_size.width / 2) as f32;
    let player_epsilon_y = (player_sprite_size.height / 2) as f32;

    let ball_epsilon_x = (ball_sprite_size.width / 2) as f32;
    let ball_epsilon_y = (ball_sprite_size.height / 2) as f32;

    let rng = rand::thread_rng();

    if ball_touch_playerone_x(ball_transform, playerone_transform, player_epsilon_x, ball_epsilon_x) {
        if same_height(ball_transform, playerone_transform, player_epsilon_y) && !ball_info.in_collision {
            info! {"ball hit player one !"}
            ball_info.direction.x = -ball_info.direction.x;
            ball_info.direction.y = -ball_info.direction.y; // should be rng but with the invert sign of y
            ball_info.in_collision = true;
        }
    } else {
        ball_info.in_collision = false;
    }

    if ball_touch_playertwo_x(ball_transform, playertwo_transform, player_epsilon_x, ball_epsilon_x) {
        if same_height(ball_transform, playertwo_transform, player_epsilon_y) && !ball_info.in_collision {
            info! {"ball hit player two !"}
            ball_info.direction.x = -ball_info.direction.x;
            ball_info.direction.y = -ball_info.direction.y; // should be rng but with the invert sign of y
            ball_info.in_collision = true;
        }
    } else {
        ball_info.in_collision = false;
    }
}

fn same_height(ball_transform: &Transform, player_transform: &Transform, player_epsilon_y: f32) -> bool {
    (ball_transform.translation.y < (player_transform.translation.y + player_epsilon_y)) && (ball_transform.translation.y > (player_transform.translation.y - player_epsilon_y))
}

fn ball_touch_playerone_x(ball_transform: &Transform, player_transform: &Transform, player_epsilon_x: f32, ball_epsilon_x: f32) -> bool {
    ((ball_transform.translation.x - ball_epsilon_x) > (player_transform.translation.x - player_epsilon_x)) && ((ball_transform.translation.x - ball_epsilon_x) < (player_transform.translation.x + player_epsilon_x))
}

fn ball_touch_playertwo_x(ball_transform: &Transform, player_transform: &Transform, player_epsilon_x: f32, ball_epsilon_x: f32) -> bool {
    ((ball_transform.translation.x + ball_epsilon_x) < (player_transform.translation.x + player_epsilon_x)) && ((ball_transform.translation.x + ball_epsilon_x) > (player_transform.translation.x - player_epsilon_x))
}

fn goal(
    mut q: Query<(&mut Transform, &mut BallInfos), With<Ball>>,
    mut score: ResMut<Score>,
    screenLimit: Res<Screen>
) {
    let (mut ball_transform, mut ball_info) = q.single_mut();
    if ball_transform.translation.x < screenLimit.left {
        score_for(&mut score, PlayerEnum::PlayerTwo);
        replace_ball_for(PlayerEnum::PlayerOne, &mut ball_transform, &mut ball_info);
    }
    if ball_transform.translation.x>  screenLimit.right {
        score_for(&mut score, PlayerEnum::PlayerOne);
        replace_ball_for(PlayerEnum::PlayerTwo, &mut ball_transform, &mut ball_info);
    }
}

fn replace_ball_for(player: PlayerEnum, ball_transform: &mut Mut<Transform>, ball_info: &mut Mut<BallInfos>) {
    ball_transform.translation.x = 0.;
    ball_transform.translation.y = 0.;

    match player {
        PlayerEnum::PlayerOne => {
            ball_info.direction = Vec3 { x: 1., y: rand::thread_rng().gen_range(-1. .. 1.), ..default() };
        }
        PlayerEnum::PlayerTwo => {
            ball_info.direction = Vec3 { x: -1., y: rand::thread_rng().gen_range(-1. .. 1.), ..default() };
        }
    }
}

fn score_for(score: &mut ResMut<Score>, player: PlayerEnum) {
    match player {
        PlayerEnum::PlayerOne => score.player1 += 1,
        PlayerEnum::PlayerTwo => score.player2 += 1,
    }
}

fn update_score_text(
    score: Res<Score>,
    mut playerOneScoreText: Query<
        &mut Text,
        (
            With<PlayerOneScoreComponent>,
            Without<PlayerTwoScoreComponent>,
        ),
    >,
    mut playerTwoScoreText: Query<
        &mut Text,
        (
            With<PlayerTwoScoreComponent>,
            Without<PlayerOneScoreComponent>,
        ),
    >,
) {
    playerOneScoreText.single_mut().sections[0].value = score.player1.to_string();
    playerTwoScoreText.single_mut().sections[0].value = score.player2.to_string();
}

fn move_player_one(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<&mut Transform, With<PlayerOne>>,
    screen_limit: Res<Screen>,
) {
    let player_speed = 300.;
    let mut transform = sprite_position.single_mut();

    if keyboard_input.pressed(KeyCode::Z) {
        if can_move_up(&transform, &screen_limit) {
            transform.translation.y += player_speed * time.delta_seconds();
        }
    }

    if keyboard_input.pressed(KeyCode::S) {
        if can_move_down(&transform, &screen_limit) {
            transform.translation.y -= player_speed * time.delta_seconds();
        }
    }
}

fn move_player_two(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<&mut Transform, With<PlayerTwo>>,
    screen_limit: Res<Screen>,
) {
    let player_speed = 300.;
    let mut transform = sprite_position.single_mut();

    if keyboard_input.pressed(KeyCode::E) {
        if can_move_up(&transform, &screen_limit) {
            transform.translation.y += player_speed * time.delta_seconds();
        }
    }

    if keyboard_input.pressed(KeyCode::D) {
        if can_move_down(&transform, &screen_limit) {
            transform.translation.y -= player_speed * time.delta_seconds();
        }
    }
}

fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("Bluemillenium_-_Rio.mp3");
    audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.5));
}

fn can_move_down(transform: &Transform, screen_limit: &Screen) -> bool {
    return transform.translation.y > screen_limit.bottom;
}

fn can_move_up(transform: &Transform, screen_limit: &Screen) -> bool {
    return transform.translation.y < screen_limit.top;
}

fn draw_screen(
    mut commands: Commands,
    screen: Res<Screen>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // draw the screen
    let length: f32 = 1000.;
    // draw top
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(length, 5.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(0., screen.top, 0.)),
        ..default()
    });

    // draw bottom
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(length, 5.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(0., screen.bottom, 0.)),
        ..default()
    });

    // draw left
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(5., length)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(screen.left, 0., 0.)),
        ..default()
    });

    // draw right
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(5., length)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(screen.right, 0., 0.)),
        ..default()
    });
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system(play_music)
        .add_startup_system_to_stage(StartupStage::PostStartup, draw_screen)
        .add_plugins(DefaultPlugins)
        .add_system(move_player_one)
        .add_system(move_player_two)
        .add_system(update_score_text)
        .add_system(goal)
        .add_system(move_ball)
        .add_system(collision_ball_top_down)
        .add_system_to_stage(CoreStage::Update, player_collision)
        .run();
}
