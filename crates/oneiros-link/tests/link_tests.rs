use oneiros_link::{Addressable, Link, LinkError};
use pretty_assertions::assert_eq;
use serde::Serialize;

#[derive(Serialize)]
struct TestAgent {
    name: String,
    persona: String,
    description: String,
}

impl Addressable for TestAgent {
    fn address_label() -> &'static str {
        "agent"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name, &self.persona))
    }
}

#[derive(Serialize)]
struct TestTexture {
    name: String,
    description: String,
    prompt: String,
}

impl Addressable for TestTexture {
    fn address_label() -> &'static str {
        "texture"
    }

    fn link(&self) -> Result<Link, LinkError> {
        Link::new(&(Self::address_label(), &self.name))
    }
}

#[test]
fn same_identity_produces_same_link() {
    let a = TestAgent {
        name: "governor.process".into(),
        persona: "process".into(),
        description: "first".into(),
    };

    let b = TestAgent {
        name: "governor.process".into(),
        persona: "process".into(),
        description: "different description".into(),
    };

    assert_eq!(a.link().unwrap(), b.link().unwrap());
}

#[test]
fn different_identity_produces_different_link() {
    let a = TestAgent {
        name: "governor.process".into(),
        persona: "process".into(),
        description: "".into(),
    };

    let b = TestAgent {
        name: "rust.expert".into(),
        persona: "expert".into(),
        description: "".into(),
    };

    assert_ne!(a.link().unwrap(), b.link().unwrap());
}

#[test]
fn different_resource_types_produce_different_links() {
    // Same "name" field value, different resource types
    let agent = TestAgent {
        name: "observation".into(),
        persona: "".into(),
        description: "".into(),
    };

    let texture = TestTexture {
        name: "observation".into(),
        description: "".into(),
        prompt: "".into(),
    };

    assert_ne!(agent.link().unwrap(), texture.link().unwrap());
}

#[test]
fn link_display_and_parse_roundtrip() {
    let agent = TestAgent {
        name: "governor.process".into(),
        persona: "process".into(),
        description: "".into(),
    };

    let link = agent.link().unwrap();
    let displayed = link.to_string();
    let parsed: Link = displayed.parse().unwrap();

    assert_eq!(link, parsed);
}

#[test]
fn link_serde_roundtrip() {
    let agent = TestAgent {
        name: "governor.process".into(),
        persona: "process".into(),
        description: "".into(),
    };

    let link = agent.link().unwrap();
    let json = serde_json::to_string(&link).unwrap();
    let deserialized: Link = serde_json::from_str(&json).unwrap();

    assert_eq!(link, deserialized);
}

#[test]
fn link_display_is_base64url() {
    let agent = TestAgent {
        name: "test".into(),
        persona: "test".into(),
        description: "".into(),
    };

    let displayed = agent.link().unwrap().to_string();

    // base64url uses only [A-Za-z0-9_-], no padding
    assert!(
        displayed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'),
        "Link display should be base64url: {displayed}"
    );
}

#[test]
fn link_deterministic_across_calls() {
    let agent = TestAgent {
        name: "governor.process".into(),
        persona: "process".into(),
        description: "anything".into(),
    };

    let links: Vec<Link> = (0..10).map(|_| agent.link().unwrap()).collect();

    for link in &links {
        assert_eq!(link, &links[0]);
    }
}
