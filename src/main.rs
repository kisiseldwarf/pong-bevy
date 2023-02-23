use bevy::{input::keyboard, prelude::*};
use bevy::ecs::system::EntityCommands;
use bevy_rapier2d::prelude::*;
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

enum Screen {
    Top,
    Bottom,
    Left,
    Right,
    None,
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

enum BallGoalHit {
    PlayerOneGoal,
    PlayerTwoGoal,
    None
}

enum PlayerEnum {
    PlayerOne,
    PlayerTwo
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Score {
        player1: 0,
        player2: 0,
    });

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
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule_y(20., 5.0))
        .insert(Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        })
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
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule_y(20., 5.0))
        .insert(Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
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
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(25.0))
        .insert(GravityScale(0.))
        .insert(Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(100., 0.),
            torque_impulse: 0.,
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

fn setup_walls(
    mut commands: Commands
) {
    commands
        .spawn(Collider::cuboid(600., 50.))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0.,
            -350.,
            0.,
        )))
        .insert(Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        });

    commands
        .spawn(Collider::cuboid(600., 50.))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0., 350., 0.,
        )))
        .insert(Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        });

    commands
        .spawn(Collider::cuboid(50., 450.))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            -600.,
            0.,
            0.,
        )))
        .insert(Sensor)
        .insert(PlayerOneWall)
        .insert(ActiveEvents::COLLISION_EVENTS);

    commands
        .spawn(Collider::cuboid(50., 450.))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            600.,
            0.,
            0.,
        )))
        .insert(PlayerTwoWall)
        .insert(Sensor)
        .insert(ActiveEvents::COLLISION_EVENTS);
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    player_one_wall_q: Query<Entity, With<PlayerOneWall>>,
    player_two_wall_q: Query<Entity, With<PlayerTwoWall>>,
    mut ball_q: Query<(&mut Transform, &mut ExternalImpulse), With<Ball>>,
    mut score: ResMut<Score>
) {
    let player_one_wall = player_one_wall_q.single();
    let player_two_wall = player_two_wall_q.single();

    let (mut ball_transform, mut ball_external_impulse) = ball_q.get_single_mut().unwrap();

    for collision_event in collision_events.iter() {
        match is_player_walls_hit(collision_event, &player_one_wall, &player_two_wall) {
            BallGoalHit::PlayerOneGoal => {
                score_for(&mut score, PlayerEnum::PlayerOne);
                replace_ball_for(PlayerEnum::PlayerTwo, &mut ball_transform, &mut ball_external_impulse);
            },
            BallGoalHit::PlayerTwoGoal => {
                score_for(&mut score, PlayerEnum::PlayerTwo);
                replace_ball_for(PlayerEnum::PlayerOne, &mut ball_transform, &mut ball_external_impulse);
            }
            BallGoalHit::None => {
                noop()
            }
        }
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn noop() {

}

fn replace_ball_for(player: PlayerEnum, ball_transform: &mut Mut<Transform>, ball_external_impulse: &mut Mut<ExternalImpulse>) {
    ball_transform.translation.x = 0.;
    ball_transform.translation.y = 0.;

    match player {
        PlayerEnum::PlayerOne => {
            ball_external_impulse.impulse = Vec2::new(200., 0.)
        }
        PlayerEnum::PlayerTwo => {
            ball_external_impulse.impulse = Vec2::new(-200., 0.)
        }
    }
}

fn score_for(score: &mut ResMut<Score>, player: PlayerEnum) {
    match player {
        PlayerEnum::PlayerOne => score.player1 += 1,
        PlayerEnum::PlayerTwo => score.player2 += 1,
    }
}

fn is_player_walls_hit(collision: &CollisionEvent, player_one_wall: &Entity, player_two_wall: &Entity) -> BallGoalHit {
    if let CollisionEvent::Started(entity_one, entity_two, _) = collision {
        info!{"First condition: {} / Second Condition: {}", player_one_wall.eq(entity_one) || player_one_wall.eq(entity_two), player_two_wall.eq(entity_one) || player_one_wall.eq(entity_two)};
        info!{"Player one wall entity: {:?}", player_one_wall}
        info!{"Player two wall entity: {:?}", player_two_wall}
        info!{"Entity one of event: {:?}", entity_one}
        info!{"Entity two of event: {:?}", entity_two}

        if player_one_wall.eq(entity_one) || player_one_wall.eq(entity_two) {
            return BallGoalHit::PlayerTwoGoal;
        } else if player_two_wall.eq(entity_one) || player_two_wall.eq(entity_two) {
            return BallGoalHit::PlayerOneGoal;
        }
    }

    return BallGoalHit::None;
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
    camera: Query<&OrthographicProjection>,
) {
    let player_speed = 300.;
    let mut transform = sprite_position.single_mut();

    if keyboard_input.pressed(KeyCode::Z) {
        if can_move_up(&transform, camera.single()) {
            transform.translation.y += player_speed * time.delta_seconds();
        }
    }

    if keyboard_input.pressed(KeyCode::S) {
        if can_move_down(&transform, camera.single()) {
            transform.translation.y -= player_speed * time.delta_seconds();
        }
    }
}

fn move_player_two(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<&mut Transform, With<PlayerTwo>>,
    camera: Query<&OrthographicProjection>,
) {
    let player_speed = 300.;
    let mut transform = sprite_position.single_mut();

    if keyboard_input.pressed(KeyCode::E) {
        if can_move_up(&transform, camera.single()) {
            transform.translation.y += player_speed * time.delta_seconds();
        }
    }

    if keyboard_input.pressed(KeyCode::D) {
        if can_move_down(&transform, camera.single()) {
            transform.translation.y -= player_speed * time.delta_seconds();
        }
    }
}

fn can_move_down(transform: &Transform, camera: &OrthographicProjection) -> bool {
    return transform.translation.y > camera.bottom;
}

fn can_move_up(transform: &Transform, camera: &OrthographicProjection) -> bool {
    return transform.translation.y < camera.top;
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_startup_system_to_stage(StartupStage::PostStartup, setup_walls)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(move_player_one)
        .add_system(move_player_two)
        .add_system(update_score_text)
        .add_system(display_events)
        .run();
}
