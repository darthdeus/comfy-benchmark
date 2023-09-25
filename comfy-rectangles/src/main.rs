use comfy::*;

simple_game!("Comfy Rectangles", update);

fn update(c: &mut EngineContext) {
    draw_circle(Vec2::ZERO, 0.5, WHITE, 0);
}
