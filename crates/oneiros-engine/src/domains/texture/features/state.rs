use crate::*;

pub struct TextureState;

impl TextureState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        match event {
            Events::Texture(TextureEvents::TextureSet(texture)) => {
                canon
                    .textures
                    .insert(texture.name.to_string(), texture.clone());
            }
            Events::Texture(TextureEvents::TextureRemoved(removed)) => {
                canon.textures.remove(&removed.name.to_string());
            }
            _ => {}
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
