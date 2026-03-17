//! Mountable implementations for resources.
//!
//! Each resource declares what it contributes to the application.
//! The Mountable impl is the integration point — it calls the
//! AppBuilder's typed collection methods to register features.

use oneiros_resource::{Feature, Mountable, Projections, Server, Tools};

use crate::app::AppBuilder;
use crate::resource_agent::Agent;
use crate::resource_level::Level;

impl Mountable<AppBuilder> for Agent {
    fn mount(&self, app: &mut AppBuilder) {
        app.nest("/agents", <Self as Feature<Server>>::feature(self));
        app.tools(<Self as Feature<Tools>>::feature(self));
        app.projections(<Self as Feature<Projections>>::feature(self));
    }
}

impl Mountable<AppBuilder> for Level {
    fn mount(&self, app: &mut AppBuilder) {
        app.nest("/levels", <Self as Feature<Server>>::feature(self));
        app.tools(<Self as Feature<Tools>>::feature(self));
        app.projections(<Self as Feature<Projections>>::feature(self));
    }
}
