use adw::prelude::*;
use counter::{build_ui, load_css, register_resources, setup_quit};
use libadwaita as adw;

fn main() -> adw::glib::ExitCode {
    register_resources();

    let app = adw::Application::builder()
        .application_id(counter::config::APP_ID)
        .build();

    app.connect_startup(|app| {
        load_css();
        setup_quit(app);
    });
    app.connect_activate(build_ui);

    app.run()
}
