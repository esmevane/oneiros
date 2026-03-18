//! Mountable implementations for resources.
//!
//! Each resource declares what it contributes to the application.
//! The Mountable impl is the integration point — it calls the
//! AppBuilder's typed collection methods to register features.

use oneiros_resource::{HasFeature, Mountable, Projections, Server, Tools};

use crate::app::AppBuilder;
use crate::resource_agent::Agent;
use crate::resource_level::Level;

impl Mountable<AppBuilder> for Agent {
    fn mount(&self, app: &mut AppBuilder) {
        app.nest("/agents", self.feature::<Server>());
        app.tools(self.feature::<Tools>());
        app.projections(self.feature::<Projections>());
    }
}

impl Mountable<AppBuilder> for Level {
    fn mount(&self, app: &mut AppBuilder) {
        app.nest("/levels", self.feature::<Server>());
        app.tools(self.feature::<Tools>());
        app.projections(self.feature::<Projections>());
    }
}
