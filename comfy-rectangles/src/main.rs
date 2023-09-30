use comfy::*;

const BORDER_COLOR: Color = BLACK;
// Workaround for poor batching with mixed WHITE and other-colored sprites.
// TODO https://github.com/bevyengine/bevy/issues/8100
const FILL_COLOR: Color = Color::rgb(1.0 - f32::EPSILON, 1.0, 1.0);

const Z_LAYERS: usize = 10;

simple_game!("Comfy Rectangles", GameState, setup, update);

pub struct GameState {
    pub count: u32,
}

impl GameState {
    pub fn new(_c: &EngineContext) -> Self {
        Self { count: 250 }
    }
}

pub struct RectangleObject {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: f32,
    pub width: f32,
    pub z_index: i32,
    pub teleport_target: f32,
}

fn update(state: &mut GameState, c: &mut EngineContext) {
    if is_mouse_button_pressed(MouseButton::Left) {
        spawn_rectangles(state, c);
        state.count *= 2;
    }

    for (_, rect) in c.world_mut().query_mut::<&mut RectangleObject>() {
        rect.position.x += rect.velocity * c.delta;

        if rect.position.x > rect.teleport_target {
            rect.position.x = -rect.position.x;
        }

        draw_sprite(
            texture_id("1px"),
            rect.position,
            FILL_COLOR,
            rect.z_index,
            rect.size,
        );
        // draw_rect(rect.position, rect.size, FILL_COLOR, rect.z_index);
        // draw_rect_outline(rect.position, rect.size, 0.1, BORDER_COLOR, rect.z_index);
    }

    draw_text(
        &format!("Count: {}", state.count),
        screen_to_world(vec2(20.0, 20.0)),
        PURPLE,
        TextAlign::TopLeft,
    );
    draw_text(
        &format!("FPS: {}", get_fps()),
        screen_to_world(vec2(20.0, 40.0)),
        PURPLE,
        TextAlign::TopLeft,
    );
}

fn setup(state: &GameState, c: &mut EngineContext) {
    c.config_mut().dev.show_fps = true;
    spawn_rectangles(state, c);

    // c.commands()
    //     .spawn((
    //         RectangleObject {
    //             velocity: rng.gen_range(60.0..120.0),
    //             width: dimensions.x,
    //             teleport_target: teleport_target - dimensions.x,
    //         },
    //         SpriteBundle {
    //             sprite: Sprite {
    //                 color: BORDER_COLOR,
    //                 custom_size: Some(dimensions),
    //                 ..default()
    //             },
    //             transform: Transform::from_xyz(
    //                 (rng.gen::<f32>() - 0.5) * width,
    //                 (rng.gen::<f32>() - 0.5) * height,
    //                 rng.gen::<f32>(),
    //             ),
    //             ..default()
    //         },
    //     ))
    //     .with_children(|children| {
    //         children.spawn(SpriteBundle {
    //             sprite: Sprite {
    //                 color: FILL_COLOR,
    //                 custom_size: Some(dimensions - Vec2::splat(3.)),
    //                 ..default()
    //             },
    //             transform: Transform::from_xyz(0., 0., f32::EPSILON),
    //             ..default()
    //         });
    //     });
}

fn spawn_rectangles(state: &GameState, c: &mut EngineContext) {
    let (width, height) = (screen_width(), screen_height());
    let world_viewport = main_camera().world_viewport();
    let teleport_target = world_viewport.x / 2.0;

    for _ in 0..state.count {
        let dimensions = splat(random_range(1.0, 5.0));

        c.commands().spawn((RectangleObject {
            position: random_box(Vec2::ZERO, world_viewport),
            // velocity: random_range(60.0, 120.0),
            velocity: random_range(6.0, 12.0),
            z_index: random_usize(0, Z_LAYERS) as i32,
            size: dimensions,
            width: dimensions.x,
            teleport_target: teleport_target - dimensions.x,
        },));
    }
}

// fn mouse_handler(
//     mut commands: Commands,
//     mouse_button_input: Res<Input<MouseButton>>,
//     window: Query<&Window, With<PrimaryWindow>>,
//     mut stats: ResMut<Stats>,
//     rectangles: Query<Entity, With<RectangleObject>>,
//     mut rng: ResMut<PseudoRng>,
// ) {
//     let Ok(window) = window.get_single() else {
//         return;
//     };
//
//     let old = stats.count;
//     if mouse_button_input.just_released(MouseButton::Left) {
//         stats.count = max(1, stats.count * 2);
//         spawn_rectangles(&mut commands, window, &mut rng.0, stats.count - old);
//     }
//     if mouse_button_input.just_released(MouseButton::Right) {
//         stats.count /= 2;
//         despawn_rectangles(&mut commands, rectangles, old - stats.count);
//     }
// }
//
//
// fn despawn_rectangles(
//     commands: &mut Commands,
//     rectangles: Query<Entity, With<RectangleObject>>,
//     num: u32,
// ) {
//     for r in rectangles.iter().take(num as usize) {
//         commands.entity(r).despawn_recursive();
//     }
// }
//
// fn bounds_updater(
//     window: Query<Entity, With<PrimaryWindow>>,
//     resize_event: Res<Events<WindowResized>>,
//     mut rectangles_query: Query<&mut RectangleObject>,
// ) {
//     let Ok(window_id) = window.get_single() else {
//         return;
//     };
//
//     let mut reader = resize_event.get_reader();
//     if let Some(e) = reader
//         .iter(&resize_event)
//         .filter(|e| e.window == window_id)
//         .last()
//     {
//         let teleport_target = -(e.width / 2.);
//         rectangles_query.par_iter_mut().for_each_mut(|mut r| {
//             r.teleport_target = teleport_target - r.width;
//         });
//     }
// }
//
// fn movement(time: Res<Time>, mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
//     rectangles_query
//         .par_iter_mut()
//         .for_each_mut(|(r, mut transform)| {
//             transform.translation.x -= r.velocity * time.delta_seconds();
//         });
// }
//
// fn collision_detection(mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
//     rectangles_query
//         .par_iter_mut()
//         .for_each_mut(|(r, mut transform)| {
//             if transform.translation.x < r.teleport_target {
//                 transform.translation.x = -transform.translation.x;
//             }
//         });
// }
