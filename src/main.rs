use ddc_hi::{Ddc, Display};
use gtk::prelude::*;
use libappindicator;
use std::path::Path;

fn main() {
    gtk::init().unwrap();

    // Tray
    let mut indicator = libappindicator::AppIndicator::new("DDC Control Tray", "");
    indicator.set_status(libappindicator::AppIndicatorStatus::Active);

    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("res");
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    indicator.set_icon_full("icon", "icon");

    // Build tray menu
    let mut m = gtk::Menu::new();

    // Brightness shortcuts
    for bri in (0..101).step_by(10) {
        let mi = gtk::CheckMenuItem::with_label(format!("{}%", bri).as_str());
        mi.connect_activate(move |_| {
            set_brightness(bri);
        });
        m.append(&mi);
    }

    // Quit
    let mi = gtk::CheckMenuItem::with_label("Quit");
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    m.append(&mi);

    indicator.set_menu(&mut m);
    m.show_all();

    gtk::main();
}

fn set_brightness(brighness: u16) {
    for mut display in Display::enumerate() {
        match display.handle.get_vcp_feature(0x10) {
            Ok(vcp_value) => {
                let vcp_value_max = vcp_value.maximum();
                let final_brighness = brighness * 100 / vcp_value_max;

                println!("Brightness final value to {:?}", final_brighness);

                display
                    .handle
                    .set_vcp_feature(0x10, final_brighness)
                    .unwrap();
            }
            Err(err) => {
                println!(
                    "failed to get value for display {:?} / {:?}: {:?}",
                    display.info.version, display.info.id, err
                );
            }
        }
    }
}
