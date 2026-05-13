use kinded::Kinded;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Kinded)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
#[kinded(kind = ProjectEventsType, display = "kebab-case")]
pub(crate) enum ProjectEvents {
    #[serde(alias = "brain-created")]
    ProjectCreated(ProjectCreated),
}

impl ProjectEvents {
    pub(crate) fn maybe_project(&self) -> Option<Project> {
        match self {
            ProjectEvents::ProjectCreated(event) => event.clone().current().ok().map(|v| v.project),
        }
    }
}

versioned! {
    pub(crate) enum ProjectCreated {
        V1 => {
            #[serde(flatten)] pub(crate) project: Project,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_types_are_kebab_cased() {
        assert_eq!(
            &ProjectEventsType::ProjectCreated.to_string(),
            "project-created"
        );
    }

    #[test]
    fn legacy_brain_created_tag_decodes_as_project_created() {
        let json = serde_json::json!({
            "type": "brain-created",
            "data": {
                "id": ProjectId::new().to_string(),
                "name": "legacy-project",
                "created_at": "2026-01-01T00:00:00Z"
            }
        });
        let event: ProjectEvents = serde_json::from_value(json).expect("decode legacy tag");
        let project = match event {
            ProjectEvents::ProjectCreated(inner) => inner.current().unwrap().project,
        };
        assert_eq!(project.name.as_str(), "legacy-project");
    }

    #[test]
    fn project_created_wire_format_is_flat() {
        let project = Project::builder()
            .name(ProjectName::new("test-project"))
            .build();

        let event = ProjectEvents::ProjectCreated(ProjectCreated::V1(ProjectCreatedV1 {
            project: project.clone(),
        }));
        let json = serde_json::to_value(&event).unwrap();

        assert_eq!(json["type"], "project-created");
        assert!(
            json["data"].get("project").is_none(),
            "flatten must elide the project envelope on the wire"
        );
        assert_eq!(json["data"]["id"], project.id.to_string());
        assert_eq!(json["data"]["name"], "test-project");
        assert!(json["data"].get("created_at").is_some());
    }
}
