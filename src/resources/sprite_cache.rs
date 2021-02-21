use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};
use anyhow::{anyhow, Result};
use std::{collections::HashMap, path::PathBuf};
use tuple_map::*;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum SpriteKey {
    Boid,
    Obstacle,
}

pub struct SpriteCache {
    sprite_map: HashMap<SpriteKey, Handle<SpriteSheet>>,
}

impl Default for SpriteCache {
    fn default() -> SpriteCache {
        SpriteCache::new()
    }
}

impl SpriteCache {
    pub fn new() -> SpriteCache {
        SpriteCache {
            sprite_map: HashMap::new(),
        }
    }

    pub fn load(&mut self, sprite_key: SpriteKey, world: &mut World) {
        let base_path = match &sprite_key {
            SpriteKey::Boid => "boids".to_string(),
            SpriteKey::Obstacle => "obstacle".to_string(),
        };

        let (img_path, ron_path) = ("png", "ron").map(|ext| {
            let mut path = PathBuf::from("sprites");
            path.push(&base_path);
            path.set_extension(ext);
            path.as_path().to_str().unwrap_or("").to_string()
        });

        // Load the texture for our sprites. We'll later need to
        // add a handle to this texture to our `SpriteRender`s, so
        // we need to keep a reference to it.
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(img_path, ImageFormat::default(), (), &texture_storage)
        };

        // Load the spritesheet definition file, which contains metadata on our
        // spritesheet texture.
        let sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                ron_path,
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            )
        };

        self.sprite_map.insert(sprite_key, sheet_handle);
    }

    pub fn fetch(&self, sprite_key: SpriteKey) -> Result<&Handle<SpriteSheet>> {
        self.sprite_map
            .get(&sprite_key)
            .ok_or_else(|| anyhow!("Sprite for sprite key {:?} is not loaded", sprite_key))
    }
}
