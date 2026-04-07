use crate::*;

pub struct TextureState;

impl TextureState {
    pub fn reduce(mut canon: BrainCanon, event: &Events) -> BrainCanon {
        if let Events::Texture(texture_event) = event {
            match texture_event {
                TextureEvents::TextureSet(texture) => {
                    canon.textures.set(texture);
                }
                TextureEvents::TextureRemoved(removed) => {
                    canon.textures.remove(&removed.name);
                }
            };
        }

        canon
    }

    pub fn reducer() -> Reducer<BrainCanon> {
        Reducer::new(Self::reduce)
    }
}
