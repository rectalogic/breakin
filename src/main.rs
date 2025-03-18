use bevy::prelude::*;
use breakin::app;

fn main() -> AppExit {
    App::new().add_plugins(app::plugin).run()
}
