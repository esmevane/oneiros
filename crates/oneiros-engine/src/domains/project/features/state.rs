use crate::*;

pub(crate) struct ProjectState;

impl ProjectState {
    pub(crate) fn reduce(mut canon: HostCanon, event: &Events) -> HostCanon {
        if let Events::Project(project_event) = event
            && let Some(project) = project_event.maybe_project()
        {
            canon.projects.set(&project);
        }

        canon
    }

    pub(crate) fn reducer() -> Reducer<HostCanon> {
        Reducer::new(Self::reduce)
    }
}
