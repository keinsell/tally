use counter::build_ui;
use gtk::prelude::*;
use gtk4 as gtk;
use libadwaita as adw;

fn find_label(window: &gtk::Window) -> Option<gtk::Label> {
    fn walk(root: &gtk::Widget) -> Option<gtk::Label> {
        if let Some(label) = root.downcast_ref::<gtk::Label>()
            && label.text() == "0"
        {
            return Some(label.clone());
        }
        let mut child = root.first_child();
        while let Some(widget) = child {
            if let Some(label) = walk(&widget) {
                return Some(label);
            }
            child = widget.next_sibling();
        }
        None
    }

    walk(window.upcast_ref())
}

#[test]
fn counting_actions_update_the_label() {
    if gtk::init().is_err() {
        eprintln!("no display available, skipping UI test");
        return;
    }
    adw::init().expect("libadwaita init");

    counter::register_resources();

    let app = adw::Application::new(
        Some("com.keinsell.Tally.Test"),
        gtk::gio::ApplicationFlags::FLAGS_NONE,
    );
    app.register(gtk::gio::Cancellable::NONE)
        .expect("app registered");

    build_ui(&app);

    let window = app.windows().pop().expect("app created a window");
    let label = find_label(&window).expect("found the count label in the built UI");
    assert_eq!(label.text(), "0");

    app.activate_action("increment", None);
    assert_eq!(label.text(), "1");

    app.activate_action("increment", None);
    app.activate_action("increment", None);
    assert_eq!(label.text(), "3");

    app.activate_action("decrement", None);
    assert_eq!(label.text(), "2");

    app.activate_action("reset", None);
    assert_eq!(label.text(), "0");
}
