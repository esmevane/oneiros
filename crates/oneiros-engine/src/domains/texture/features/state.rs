use crate::*;

pub struct TextureState;

impl TextureState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Texture(texture_event) = event {
            match texture_event {
                TextureEvents::TextureSet(setting) => {
                    if let Ok(current) = setting.current() {
                        canon.textures.set(&current.texture);
                    }
                }
                TextureEvents::TextureRemoved(removal) => {
                    if let Ok(current) = removal.current() {
                        canon.textures.remove(&current.name);
                    }
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
