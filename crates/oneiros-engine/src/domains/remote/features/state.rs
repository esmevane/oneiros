use crate::*;

#[allow(dead_code)]
pub(crate) struct RemoteState;

#[allow(dead_code)]
impl RemoteState {
    pub(crate) fn reduce(mut canon: HostCanon, event: &Events) -> HostCanon {
        if let Events::Remote(remote_event) = event {
            match remote_event {
                RemoteEvents::RemoteAdded(added) => {
                    if let Ok(current) = added.current() {
                        canon.remotes.set(&current.remote);
                    }
                }
                RemoteEvents::RemoteRemoved(removed) => {
                    if let Ok(current) = removed.current() {
                        canon.remotes.remove(&current.id);
                    }
                }
            };
        }
        canon
    }

    pub(crate) fn reducer() -> Reducer<HostCanon> {
        Reducer::new(Self::reduce)
    }
}
