use std::cell::Cell;
use std::rc::Rc;

pub mod config;

use adw::prelude::*;
use gtk::{gio, glib};
use gtk4 as gtk;
use libadwaita as adw;

pub fn register_resources() {
    let resources = gio::Resource::from_data(&glib::Bytes::from_static(config::COUNTER_RESOURCES))
        .expect("failed to load resources");
    gio::resources_register(&resources);
}

pub fn load_css() {
    let display = gtk::gdk::Display::default().expect("could not connect to a display");

    // Make the icons bundled in the GResource visible to Gtk.IconTheme under
    // the app id, so Adw.AboutDialog:application-icon resolves even when the
    // app is run uninstalled (e.g. `cargo run`). Once installed, the
    // hicolor theme provides the same icons from the system data dir.
    gtk::IconTheme::for_display(&display)
        .add_resource_path(&format!("{}icons", config::RESOURCE_BASE));

    let provider = gtk::CssProvider::new();
    provider.load_from_resource(&format!("{}style.css", config::RESOURCE_BASE));

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn show_about(parent: &adw::ApplicationWindow) {
    // Standard GNOME pattern: About is a transient overlay dialog, not a page
    // inside the main window. It sizes itself independently, so the main
    // window can stay small (400x300). On narrow/mobile it adapts to a
    // bottom sheet automatically.
    let dialog = adw::AboutDialog::builder()
        .application_name("Tally")
        .application_icon(config::APP_ID)
        .version(config::VERSION)
        .developers(["keinsell"])
        // Icon artwork by Victoria Lacroix, taken from Tally
        // (https://codeberg.org/bragefuglseth/tally), GPL-3.0-or-later.
        .designers(["Victoria Lacroix"])
        .website("https://github.com/keinsell/tally")
        .issue_url("https://github.com/keinsell/tally/issues")
        .license_type(gtk::License::MitX11)
        .build();
    dialog.present(Some(parent));
}

pub fn setup_quit(app: &adw::Application) {
    let action = gio::SimpleAction::new("quit", None);
    {
        let weak_app = app.downgrade();
        action.connect_activate(move |_, _| {
            if let Some(app) = weak_app.upgrade() {
                app.quit();
            }
        });
    }
    app.add_action(&action);
}

pub fn build_ui(app: &adw::Application) {
    let builder = gtk::Builder::from_resource(&format!("{}counter.ui", config::RESOURCE_BASE));

    let window = builder
        .object::<adw::ApplicationWindow>("counter-window")
        .unwrap();
    let count_label = builder.object::<gtk::Label>("count-label").unwrap();
    let menu_button = builder.object::<gtk::MenuButton>("menu-button").unwrap();

    let count = Rc::new(Cell::new(0i64));

    let render: Rc<dyn Fn(i64)> = {
        let count_label = count_label.clone();
        Rc::new(move |value: i64| {
            count_label.set_label(&value.to_string());
            if value == 0 {
                count_label.add_css_class("dim-label");
            } else {
                count_label.remove_css_class("dim-label");
            }
        })
    };

    let counting_action =
        |name: &str, delta: i64, count: Rc<Cell<i64>>, render: Rc<dyn Fn(i64)>| {
            let action = gio::SimpleAction::new(name, None);
            action.connect_activate(move |_, _| {
                let value = if delta == 0 {
                    0
                } else {
                    count.get().checked_add(delta).unwrap_or(if delta < 0 {
                        i64::MIN
                    } else {
                        i64::MAX
                    })
                };
                count.set(value);
                render(value);
            });
            action
        };

    let decrement = counting_action("decrement", -1, count.clone(), render.clone());
    let reset = counting_action("reset", 0, count.clone(), render.clone());
    let increment = counting_action("increment", 1, count.clone(), render.clone());

    app.add_action(&decrement);
    app.add_action(&reset);
    app.add_action(&increment);

    let about = gio::SimpleAction::new("about", None);
    {
        let window = window.clone();
        about.connect_activate(move |_, _| {
            show_about(&window);
        });
    }
    window.add_action(&about);

    let menu = gio::Menu::new();
    menu.append(Some("_About Tally"), Some("win.about"));
    menu_button.set_popover(Some(&gtk::PopoverMenu::from_model(Some(&menu))));

    app.set_accels_for_action(
        "app.increment",
        &["<Primary>plus", "<Primary>equal", "plus"],
    );
    app.set_accels_for_action("app.decrement", &["<Primary>minus", "minus"]);
    app.set_accels_for_action("app.reset", &["<Primary>0"]);
    app.set_accels_for_action("app.quit", &["<Primary>q"]);

    window.set_application(Some(app));
    window.present();
}
