use crate::*;

pub enum ContinuityDocs {
    Emerge,
    Status,
    Recede,
    Wake,
    Dream,
    Introspect,
    Reflect,
    Sense,
    Sleep,
    Guidebook,
}

impl ContinuityDocs {
    pub fn tag(&self) -> Tag {
        Tag::builder()
            .name("continuity")
            .description("Agent lifecycle and cognitive operations")
            .build()
    }

    pub fn resource_docs(&self) -> ResourceDocs {
        let tag = self.tag();
        match self {
            Self::Emerge => ResourceDocs::builder()
                .tag(tag)
                .nickname("emerge-agent")
                .summary("Emerge an agent")
                .description("Bring an agent online for the first time, establishing its initial cognitive presence.")
                .build(),
            Self::Status => ResourceDocs::builder()
                .tag(tag)
                .nickname("status-agent")
                .summary("Get agent status")
                .description("Report the current lifecycle state and cognitive activity level of an agent.")
                .build(),
            Self::Recede => ResourceDocs::builder()
                .tag(tag)
                .nickname("recede-agent")
                .summary("Recede an agent")
                .description("Gracefully withdraw an agent from active service, preserving its accumulated context.")
                .build(),
            Self::Wake => ResourceDocs::builder()
                .tag(tag)
                .nickname("wake-agent")
                .summary("Wake an agent")
                .description("Restore a dormant agent to an active state, making it ready to receive work.")
                .build(),
            Self::Dream => ResourceDocs::builder()
                .tag(tag)
                .nickname("dream-agent")
                .summary("Dream an agent")
                .description("Generate a cognitive context document from the agent's accumulated thoughts and memories.")
                .build(),
            Self::Introspect => ResourceDocs::builder()
                .tag(tag)
                .nickname("introspect-agent")
                .summary("Introspect an agent")
                .description("Summarize the agent's current session into consolidated memories before context compaction.")
                .build(),
            Self::Reflect => ResourceDocs::builder()
                .tag(tag)
                .nickname("reflect-agent")
                .summary("Reflect an agent")
                .description("Capture a snapshot of the agent's present reasoning as a durable reflection.")
                .build(),
            Self::Sense => ResourceDocs::builder()
                .tag(tag)
                .nickname("sense-agent")
                .summary("Sense an agent")
                .description("Sample the agent's current cognitive pressure and activity without modifying its state.")
                .build(),
            Self::Sleep => ResourceDocs::builder()
                .tag(tag)
                .nickname("sleep-agent")
                .summary("Sleep an agent")
                .description("Suspend an agent into a dormant state, pausing cognitive activity while preserving context.")
                .build(),
            Self::Guidebook => ResourceDocs::builder()
                .tag(tag)
                .nickname("guidebook-agent")
                .summary("Get agent guidebook")
                .description("Retrieve the accumulated guidance document that shapes the agent's cognitive style and constraints.")
                .build(),
        }
    }
}
