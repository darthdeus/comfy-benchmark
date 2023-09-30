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

    for i in 1..=100 {
        textures.push((format!("comfy-{}", i), format!("comfy-{}.png", i)));

        sounds.push((format!("music-{}", i), format!("music-{}.wav", i)));
    }

    load_multiple_textures(textures);

    load_multiple_sounds(sounds);
}

fn setup(_c: &mut EngineContext) {
    load_assets();
}

fn update(_c: &mut EngineContext) {}
