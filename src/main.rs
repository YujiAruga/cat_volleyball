use bevy::prelude::*;
use bevy::sprite::{TextureAtlas, TextureAtlasSprite};

const ARENA_WIDTH: f32 = 700.0;
const ARENA_HEIGHT: f32 = 700.0;

const PLAYER_HEIGHT: f32 = 32.0;
const PLAYER_WIDTH: f32 = 22.0;

const PLAYER_SPEED: f32 = 60.0;

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

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let spritesheet = asset_server.load(
        "textures/spritesheet.png"
    );

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

    let texture_atlas_handle = texture_atlases.add(sprite_atlas);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            ARENA_WIDTH/2.0,
            ARENA_HEIGHT/2.0,
            1.0
        ),
        ..default()
    });

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
        .add_system(player)
        .run();
}
