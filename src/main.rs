use bevy::prelude::*;
use bevy_Tic_Tac_Toe::state::GameState;
#[derive(Resource)]
struct Turn(bool); // false = X, true = O

#[derive(Resource)]
struct Board([[char; 3]; 3]);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_grid)
        .insert_resource(Turn(false))
        .insert_resource(Board([[' '; 3]; 3]))
        .add_systems(Update, click_spawn_circle)
        // .add_systems(Startup, setup)
        // .add_systems(Update, update_bloom_settings)
        .run();
}

fn setup(mut commands: Commands) {
    Triangle2d::new(
        Vec2::Y * 50.0,
        Vec2::new(-50.0, -50.0),
        Vec2::new(50.0, -50.0),
    );
    Annulus::new(10.0, 10.0);

    commands.spawn(Camera2d::default());
}

const CELL: f32 = 120.0; // ขนาดของแต่ละช่อง
const THICK: f32 = 6.0; // ความหนาเส้น

fn spawn_grid(mut commands: Commands) {
    let line_color = Color::srgb(0.8, 0.8, 0.8);
    let half = CELL / 2.0;

    // ------------ เส้นตั้ง 2 เส้น ------------
    for x in [-half, half] {
        commands.spawn((
            Transform::from_xyz(x, 0.0, 1.0),
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(THICK, CELL * 3.0)),
                ..default()
            },
        ));
    }

    // ------------ เส้นนอน 2 เส้น ------------
    for y in [-half, half] {
        commands.spawn((
            Transform::from_xyz(0.0, y, 1.0),
            Sprite {
                color: line_color,
                custom_size: Some(Vec2::new(CELL * 3.0, THICK)),
                ..default()
            },
        ));
    }
}

fn click_spawn_circle(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut turn: ResMut<Turn>,
    mut board: ResMut<Board>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (camera, cam_tf) = camera_q.single().unwrap();
    let ray = camera.viewport_to_world(cam_tf, cursor_pos).unwrap();

    let world = ray.origin.truncate();

    // === 1) คำนวณ cell index ===
    let half_board = CELL * 1.5; // 3 ช่อง
    if world.x < -half_board
        || world.x > half_board
        || world.y < -half_board
        || world.y > half_board
    {
        return; // นอกกระดาน
    }

    let col = ((world.x + half_board) / CELL).floor() as usize;
    let row = ((world.y + half_board) / CELL).floor() as usize;

    if row > 2 || col > 2 {
        return;
    }
    //  เป็น array 2D ปกติ แต่ไป เทียบ กับ space ของ cell ในการเช็ค
    // === 2) เช็คว่า cell ว่างไหม ===
    if board.0[row][col] != ' ' {
        return; // มีแล้ว ห้ามเขียนซ้ำ
    }

    // === 3) คำนวณตำแหน่ง center ของ cell ===
    let cell_center_x = -half_board + (col as f32) * CELL + CELL / 2.0;
    let cell_center_y = -half_board + (row as f32) * CELL + CELL / 2.0;

    // === 4) วาง X หรือ O ===
    let symbol = if turn.0 { 'O' } else { 'X' };
    board.0[row][col] = symbol;

    match symbol {
        'X' => {
            // ใช้ Sprite เป็น X
            commands.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(80., 10.)),
                Transform::from_xyz(cell_center_x, cell_center_y, 10.0)
                    .with_rotation(Quat::from_rotation_z(0.8)),
            ));
            commands.spawn((
                Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(80., 10.)),
                Transform::from_xyz(cell_center_x, cell_center_y, 10.0)
                    .with_rotation(Quat::from_rotation_z(-0.8)),
            ));
        }
        'O' => {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(70.)),
                Transform::from_xyz(cell_center_x, cell_center_y, 10.0),
            ));
        }
        _ => {}
    }

    // === 5) สลับตา ===
    turn.0 = !turn.0;
}
