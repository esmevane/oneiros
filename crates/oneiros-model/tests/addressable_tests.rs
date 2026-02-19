use oneiros_link::Addressable;
use oneiros_model::*;
use pretty_assertions::assert_eq;

#[test]
fn cross_type_discrimination() {
    // "observation" as a texture vs "observation" as a sensation
    let texture = Texture {
        name: TextureName::new("observation"),
        description: Description::default(),
        prompt: Prompt::default(),
    };

    let sensation = Sensation {
        name: SensationName::new("observation"),
        description: Description::default(),
        prompt: Prompt::default(),
    };

    assert_ne!(texture.link().unwrap(), sensation.link().unwrap());
}

#[test]
fn link_roundtrip_through_display() {
    let agent = Agent {
        name: AgentName::new("governor.process"),
        persona: PersonaName::new("process"),
        description: Description::default(),
        prompt: Prompt::default(),
    };

    let link = agent.link().unwrap();
    let displayed = link.to_string();
    let parsed: oneiros_link::Link = displayed.parse().unwrap();

    assert_eq!(link, parsed);
}
