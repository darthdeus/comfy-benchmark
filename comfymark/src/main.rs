use comfy::*;

simple_game!("Comfymark FLOAT", setup, update);

pub struct Comf {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub color: Color,
}

static STATE: Lazy<AtomicRefCell<Vec<Comf>>> = Lazy::new(|| AtomicRefCell::new(Vec::new()));

fn setup(_c: &mut EngineContext) {
    game_config_mut().dev.show_fps = true;
    let viewport = main_camera().world_viewport();
    let mut state = STATE.borrow_mut();
    let colors = vec![RED, GREEN, BLUE, MAGENTA, YELLOW, CYAN];

    for _ in 0..35000 {
        state.push(Comf {
            position: random_box(Vec2::ZERO, viewport),
            velocity: random_dir() * 10.0,
            color: *colors.choose().unwrap(),
            size: splat(1.0),
        });
    }
}

fn update(_c: &mut EngineContext) {
    let viewport = main_camera().world_viewport() / 2.0;
    let delta = delta();
    const GRAVITY: f32 = -10.0;

    for comf in STATE.borrow_mut().iter_mut() {
        comf.position += comf.velocity * delta;
        comf.velocity.y += GRAVITY * delta;

        let hx = comf.size.x / 2.0;
        let hy = comf.size.y / 2.0;

        let x = comf.position.x;
        let y = comf.position.y;
        let vx = comf.velocity.x;
        let vy = comf.velocity.y;

        if (vx > 0.0 && x + hx > viewport.x) || (vx <= 0.0 && x - hx < -viewport.x) {
            comf.velocity.x = -vx;
        }

        if vy < 0.0 && y - hy < -viewport.y {
            comf.velocity.y = -vy;
        }

        draw_comfy(comf.position, comf.color, 0, comf.size);
    }

    let top_left = main_camera().screen_top_left();
    draw_rect(top_left + vec2(1.0, -0.5), vec2(2.5, 1.0), BLACK, 5);

    draw_text(
        &format!("{} FPS", get_fps()),
        top_left,
        GREEN,
        TextAlign::TopLeft,
    );
    draw_text(
        &format!("{} comfs", STATE.borrow().len()),
        top_left + vec2(0.0, -0.5),
        GREEN,
        TextAlign::TopLeft,
    );
}
