use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasSprite};
use rand::Rng;

const ARENA_WIDTH: f32 = 700.0;
const ARENA_HEIGHT: f32 = 700.0;

const PLAYER_HEIGHT: f32 = 32.0;
const PLAYER_WIDTH: f32 = 22.0;

const PLAYER_SPEED: f32 = 60.0;

const BALL_VELOCITY_X: f32 = 30.0;
const BALL_VELOCITY_Y: f32 = 0.0;
const BALL_RADIUS: f32 = 4.0;

#[derive(Copy, Clone)]
enum Side {
    Left,
    Right,
}

impl Side {
    // Get keycode for move left
    fn go_left_key(&self) -> KeyCode {
        match self {
            Side::Left => KeyCode::A,
            Side::Right => KeyCode::Left,
        }
    }

    // Get keycode for move right
    fn go_right_key(&self) -> KeyCode {
        match self {
            Side::Left => KeyCode::D,
            Side::Right => KeyCode::Right,
        }
    }

    // Determine the permissible range of the cat
    fn range(&self) -> (f32, f32) {
        match self {
            Side::Left => (
                PLAYER_WIDTH / 2.0,
                ARENA_WIDTH / 2.0 - PLAYER_WIDTH / 2.0
            ),
            Side::Right => (
                ARENA_WIDTH / 2.0 + PLAYER_WIDTH / 2.0,
                ARENA_WIDTH - PLAYER_WIDTH / 2.0,
            ),
        }

    }
}

#[derive(Component)]
struct Player {
    side: Side,
}

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec2,
    pub radius: f32,
}


/*
Bounce system
 */

fn point_in_rect(
    x: f32, // ball's x and y location
    y: f32,
    left: f32, // the player box's boundary
    bottom: f32,
    right: f32,
    top: f32,
) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn bounce(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
) {
    for (mut ball, ball_transform) in ball_query.iter_mut() {
        let ball_x = ball_transform.translation.x;
        let ball_y = ball_transform.translation.y;

        if ball_y <= ball.radius && ball.velocity.y < 0.0 {
            ball.velocity.y = -ball.velocity.y;
        }
        else if ball_y >= (ARENA_HEIGHT - ball.radius) && ball.velocity.y > 0.0 {
            ball.velocity.y = -ball.velocity.y;
        }
        else if ball_x <= ball.radius && ball.velocity.x < 0.0 {
            ball.velocity.x = -ball.velocity.x;
        }
        else if ball_x >= (ARENA_WIDTH - ball.radius) && ball.velocity.x > 0.0 {
            ball.velocity.x = -ball.velocity.x;
        }
        // ... additional collision detection

        for (player, player_trans) in player_query.iter() {
            let player_x = player_trans.translation.x;
            let player_y = player_trans.translation.y;

            if point_in_rect(
                ball_x,
                ball_y,
                player_x - PLAYER_WIDTH / 2.0 - ball.radius,
                player_y - PLAYER_HEIGHT / 2.0 - ball.radius,
                player_x + PLAYER_WIDTH / 2.0 + ball.radius,
                player_y + PLAYER_HEIGHT / 2.0 + ball.radius,
            ) {
                if ball.velocity.y < 0.0 {
                    // Only bounce when ball is falling
                    ball.velocity.y = -ball.velocity.y;

                    let mut rng = rand::thread_rng();
                    match player.side {
                        Side::Left => {
                            ball.velocity.x = ball.velocity.x.abs() * rng.gen_range(0.6..1.4)
                        }
                        Side::Right => {
                            ball.velocity.x = -ball.velocity.x.abs() * rng.gen_range(0.6..1.4)
                        }
                    }
                }
            }
        }
    }
}


fn move_ball(
    time: Res<Time>,
    mut query: Query<(&mut Ball, &mut Transform)>
) {
    for (mut ball, mut transform) in query.iter_mut() {
        // Apply movement deltas
        transform.translation.x += ball.velocity.x * time.raw_delta_seconds();
        transform.translation.y += (
            ball.velocity.y
                + time.raw_delta_seconds()
                * GRAVITY_ACCELERATION / 2.0)
            * time.raw_delta_seconds();
        ball.velocity.y += time.raw_delta_seconds() * GRAVITY_ACCELERATION;
    }
}


fn initialize_player(
    commands: &mut Commands,
    atlas: Handle<TextureAtlas>,
    cat_sprite: usize,
    side: Side,
    x: f32,
    y: f32,
) {
    commands.spawn((
        Player { side },
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(cat_sprite),
            texture_atlas: atlas,
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
        ));
}

fn player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let left = if keyboard_input.pressed(
            player.side.go_left_key())
        {
            -1.0f32
        }
        else {
            0.0
        };

        let right = if keyboard_input.pressed(
            player.side.go_right_key())
        {
            1.0f32
        }
        else {
            0.0
        };

        let direction = left + right;
        let offset = direction * PLAYER_SPEED * time.raw_delta_seconds();

        // Apply movement deltas
        transform.translation.x += offset;
        let (left_limit, right_limit) = player.side.range();
        transform.translation.x = transform.translation.x.clamp(
            left_limit, right_limit
        );
    }
}


// Initializes one ball in the middle-ish of the arena.
fn initialize_ball(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas: Handle<TextureAtlas>,
    ball_sprite: usize,
) {
    commands.spawn((
        Ball {
            velocity: Vec2::new(BALL_VELOCITY_X, BALL_VELOCITY_Y),
            radius: BALL_RADIUS,
        },
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(ball_sprite),
            texture_atlas: atlas,
            transform: Transform::from_xyz(
                ARENA_WIDTH / 2.0,
                ARENA_HEIGHT / 2.0,
                0.0
            ),
            ..default()
        },
        ));
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet = asset_server.load(
        "textures/spritesheet.png"
    );


    /*
    Set up cat indicies
     */
    let mut sprite_atlas = TextureAtlas::new_empty(
        spritesheet,
        Vec2::new(58.0, 34.0)
    );

    let left_cat_corner = Vec2::new(11.0, 1.0);
    let right_cat_corner = Vec2::new(35.0, 1.0);
    let cat_size = Vec2::new(22.0, 32.0);

    let left_cat_index = sprite_atlas.add_texture(
        Rect::from_corners(
            left_cat_corner,
            left_cat_corner + cat_size,
        )
    );

    let right_cat_index = sprite_atlas.add_texture(
        Rect::from_corners(
            right_cat_corner,
            right_cat_corner + cat_size,
        )
    );

    /*
    Initialise a Ball
     */
    let ball_corner = Vec2::new(1.0, 1.0);
    let ball_size = Vec2::new(8.0, 8.0);

    // create texture atlas handle.
    let ball_index =
        sprite_atlas.add_texture(Rect::from_corners(
            ball_corner,
            ball_corner + ball_size
        ));


    /*
    Initialise cameras
     */
    let texture_atlas_handle = texture_atlases.add(sprite_atlas);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            ARENA_WIDTH/2.0,
            ARENA_HEIGHT/2.0,
            1.0
        ),
        ..default()
    });

    initialize_ball(
        &mut commands,
        &asset_server,
        texture_atlas_handle.clone(),
        ball_index,
    );


    /*
    Initialise Players
     */
    initialize_player(
        &mut commands,
        texture_atlas_handle.clone(),
        left_cat_index,
        Side::Left,
        PLAYER_WIDTH / 2.0,
        PLAYER_HEIGHT / 2.0,
    );
    initialize_player(
        &mut commands,
        texture_atlas_handle,
        right_cat_index,
        Side::Right,
        ARENA_WIDTH - PLAYER_WIDTH / 2.0,
        PLAYER_HEIGHT / 2.0,
    );
}


/*
Ball movement system
 */

pub const GRAVITY_ACCELERATION: f32 = -40.0;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Cat Volleyball".into(),
            resolution: (ARENA_WIDTH, ARENA_HEIGHT).into(),
            ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .add_system(move_ball)
        .add_system(bounce)
        .add_system(player)
        .run();
}
