use crate::*;

pub(crate) struct TextureState;

impl TextureState {
    pub(crate) fn reduce(mut canon: ProjectCanon, event: &Events) -> ProjectCanon {
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

    pub(crate) fn reducer() -> Reducer<ProjectCanon> {
        Reducer::new(Self::reduce)
    }
}
