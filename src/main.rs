use bevy::{prelude::*, input::keyboard};
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

#[derive(Component)]
enum DirectionUpAndDown {
    Up,
    Down,
}

#[derive(Resource)]
struct Score {
    player1: i32,
    player2: i32
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
    velocity: f32
}

#[derive(Component)]
struct TitleComponent;

#[derive(Component)]
struct PlayerOneScoreComponent;

#[derive(Component)]
struct PlayerTwoScoreComponent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Score {
        player1: 0,
        player2: 0
    });

    // camera
    commands.spawn(Camera2dBundle::default());
    
    // player one
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Pong.png"),
            transform: Transform::from_xyz(-250., 0., 0.),
            ..default()
        },
        DirectionUpAndDown::Up,
        PlayerOne,
    ))
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(50.,50.))
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
    });;

    // player two
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Pong.png"),
            transform: Transform::from_xyz(250., 0., 0.),
            ..default()
        },
        DirectionUpAndDown::Up,
        PlayerTwo,
    ))
    .insert(RigidBody::KinematicPositionBased)
    .insert(Collider::cuboid(50., 50.))
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
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Balle.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Ball,
    ))
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(50.0))
    .insert(GravityScale(0.))
    .insert(Restitution {
        coefficient: 1.,
        combine_rule: CoefficientCombineRule::Max,
    })
    .insert(ExternalImpulse {
        impulse: Vec2::new(150., 0.),
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

    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(25.)),
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::NoWrap,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.15, 0.15, 0.15, 0.)),
            ..default()
        }
    )).with_children(|global_container| {

        // player one score
        global_container.spawn(NodeBundle {
            .. default()
        }).with_children(|player_one_container| {
            player_one_container.spawn((
                TextBundle::from_section(
                    0.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    }
                ),
                PlayerOneScoreComponent
            ));
        });

        // score title
        global_container.spawn(NodeBundle { ..default()}).with_children(|score_title_container| {
            score_title_container.spawn((
                TextBundle::from_section(
                    "Score",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    }
                ).with_style(Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    flex_grow: 2.,
                    ..default()
                }),
                TitleComponent
            ));
        });

        // player two score
        global_container.spawn(NodeBundle { ..default() }).with_children(|player_two_score_container| {
            player_two_score_container.spawn((
                TextBundle::from_section(
                    0.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE
                    }),
                PlayerTwoScoreComponent
            ));
        });
    });

    
}

fn ball_move(time: Res<Time>, mut ball_position: Query<(&mut Transform, &mut BallDirection)>) {
    // ball_position.single_mut().0.translation.x += ball_position.single().1.direction.normalize().x * ball_position.single().1.velocity * time.delta_seconds();
    // ball_position.single_mut().0.translation.y += ball_position.single().1.direction.normalize().y * ball_position.single().1.velocity * time.delta_seconds();
}

fn update_score_text(
    score: Res<Score>,
    mut playerOneScoreText: Query<&mut Text, (With<PlayerOneScoreComponent>, Without<PlayerTwoScoreComponent>)>,
    mut playerTwoScoreText: Query<&mut Text, (With<PlayerTwoScoreComponent>, Without<PlayerOneScoreComponent>)>,
) {
    playerOneScoreText.single_mut().sections[0].value = score.player1.to_string();
    playerTwoScoreText.single_mut().sections[0].value = score.player2.to_string();
}

fn ball_collision(
    mut score: ResMut<Score>,
    camera: Query<&OrthographicProjection>,
    player_one: Query<(&Transform), With<PlayerOne>>, 
    player_two: Query<(&Transform), With<PlayerTwo>>,
    mut ball_position: Query<(&mut BallDirection, &mut Transform), (Without<PlayerOne>, Without<PlayerTwo>)>, 
)
{
//  let mut rng = rand::thread_rng();
//     if ball_collide_with_player(
//         ball_position.single().2,
//         ball_position.single().1.size,
//         player_one.single().0,
//         player_one.single().1.size) {
//         ball_position.single_mut().0.direction = Vec2::new(1., rng.gen_range(-1.0..1.));
//     } else if ball_collide_with_player(
//         ball_position.single().2,
//         ball_position.single().1.size,
//         player_two.single().0,
//         player_two.single().1.size) {
//         ball_position.single_mut().0.direction = Vec2::new(-1., rng.gen_range(-1.0..1.));
//     }
    
    // match ball_collide_with_screen(ball_position.single().2, camera.single()) {
    //     Screen::Top => ball_position.single_mut().0.direction = Vec2::new(rng.gen_range(-1.0..1.0), -1.),
    //     Screen::Bottom => ball_position.single_mut().0.direction = Vec2::new(rng.gen_range(-1.0..1.0), 1.),
    //     Screen::Left => {
    //         increase_player_two_score(&mut score);
    //         ball_position.single_mut().2.translation = Vec3::new(0.,0.,0.);
    //         ball_position.single_mut().0.direction = Vec2::new(1., rng.gen_range(-1.0..1.0))
    //     },
    //     Screen::Right => {
    //         increase_player_one_score(&mut score);
    //         ball_position.single_mut().2.translation = Vec3::new(0.,0.,0.);
    //         ball_position.single_mut().0.direction = Vec2::new(-1., rng.gen_range(-1.0..1.0))
    //     },
    //     Screen::None => {}
    // }
}

fn increase_player_one_score(score: &mut Score) {
    score.player1 += 1;
}

fn increase_player_two_score(score: &mut Score) {
    score.player2 += 1;
}

fn ball_collide_with_screen(ball_transform: &Transform, camera: &OrthographicProjection) -> Screen {
    let translation = ball_transform.translation;
    return if translation.y > camera.top { Screen::Top }
    else if translation.y < camera.bottom { Screen::Bottom }
    else if translation.x > camera.right { Screen::Right }
    else if translation.x < camera.left { Screen::Left }
    else { Screen::None };
}

/** fn ball_collide_with_player(ball_transform: &Transform, ball_colliding_size: f32, player_transform: &Transform, player_colliding_size: f32) -> bool {
    return Collider::from(player_transform, player_colliding_size).collide(&Collider::from(ball_transform, ball_colliding_size));
} **/

fn playerOne_input(time: Res<Time>, mut sprite_position: Query<(&mut DirectionUpAndDown, &mut Transform, With<PlayerOne>)>, camera: Query<&OrthographicProjection>) {
    for (mut logo, mut transform, _) in &mut sprite_position {
        match *logo {
            DirectionUpAndDown::Up => {
                if can_move_up(&transform, camera.single()) {
                    transform.translation.y += 150. * time.delta_seconds();
                }
            },
            DirectionUpAndDown::Down => {
                if can_move_down(&transform, camera.single()) { 
                    transform.translation.y -= 150. * time.delta_seconds();
                }
            },
        }
    }
}

fn playerTwo_input(time: Res<Time>, mut sprite_position: Query<(&mut DirectionUpAndDown, &mut Transform, With<PlayerTwo>)>, camera: Query<&OrthographicProjection>) {
    for (mut logo, mut transform, _) in &mut sprite_position {
        match *logo {
            DirectionUpAndDown::Up => {
                if can_move_up(&transform, camera.single()) {
                    transform.translation.y += 150. * time.delta_seconds();
                }
            },
            DirectionUpAndDown::Down => {
                if can_move_down(&transform, camera.single()) { 
                    transform.translation.y -= 150. * time.delta_seconds();
                }
            },
        }
    }
}

fn draw_pOne(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut DirectionUpAndDown, &mut Transform, With<PlayerOne>)>) {
    for item in keyboard_input.get_just_pressed().into_iter() {
        info!("{:#?}", item);
    }

    if keyboard_input.pressed(KeyCode::Z) {
        *query.single_mut().0 = DirectionUpAndDown::Up;
    }

    if keyboard_input.pressed(KeyCode::S) {
        *query.single_mut().0 = DirectionUpAndDown::Down;
    }
}

fn draw_pTwo(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut DirectionUpAndDown, &mut Transform, With<PlayerTwo>)>) {
    for item in keyboard_input.get_just_pressed().into_iter() {
        info!("{:#?}", item);
    }

    if keyboard_input.pressed(KeyCode::E) {
        *query.single_mut().0 = DirectionUpAndDown::Up;
    }

    if keyboard_input.pressed(KeyCode::D) {
        *query.single_mut().0 = DirectionUpAndDown::Down;
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
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_system(ball_move)
    .add_system(ball_collision)
    .add_system(playerOne_input)
    .add_system(draw_pOne)
    .add_system(playerTwo_input)
    .add_system(draw_pTwo)
    .add_system(update_score_text)
    .run();
}
