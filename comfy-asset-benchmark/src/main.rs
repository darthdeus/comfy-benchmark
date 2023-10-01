use comfy::*;

simple_game!("Comfymark FLOAT", setup, update);

pub static ASSET_DIR: include_dir::Dir<'_> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets");

fn base_path(path: &str) -> String {
    format!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/{}"), path)
}

pub fn load_assets() {
    init_asset_source(&ASSET_DIR, base_path);

    let mut textures = Vec::new();
    let mut sounds = Vec::new();

    for i in 1..=50 {
        textures.push((format!("comfy-{}", i), format!("comfy-{}.png", i)));
        textures.push((format!("wall-{}", i), format!("wall-{}.jpg", i)));
        sounds.push((format!("music-{}", i), format!("music-{}.wav", i)));
    }

    load_multiple_textures(textures);
    load_multiple_sounds(sounds);
}

fn setup(_c: &mut EngineContext) {
    load_assets();
}

fn update(_c: &mut EngineContext) {
    draw_text("The wedge doesn't represent loading progress. It's here to show that loading is async and doesn't halt the game",
        vec2(0.0, -3.0), WHITE, TextAlign::Center);

    // px() gives us world-space size of 1 pixel
    draw_arc_wedge(Vec2::ZERO, 2.0, 2.0 * px(), 0.0, get_time() as f32, PINK, 0);

    draw_text(
        &format!(
            "Loaded assets: {}/{}",
            assets_loaded(),
            assets_queued_total()
        ),
        vec2(0.0, 3.0),
        WHITE,
        TextAlign::Center,
    );

    draw_sprite(texture_id("comfy-10"), vec2(5.0, 0.0), WHITE, 0, splat(3.0));
    draw_sprite(texture_id("wall-10"), vec2(-5.0, 0.0), WHITE, 0, splat(3.0));
}
