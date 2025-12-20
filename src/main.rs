use anyhow::anyhow;
use ddc_hi::{Ddc, Display};
use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use tempdir::TempDir;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");
const SET_MAX_RETRIES: usize = 3;
static DISPLAYS: Lazy<Mutex<Vec<Display>>> = Lazy::new(|| Mutex::new(Vec::new()));

fn main() {
    if let Err(e) = reload_displays() {
        panic!("failed to load displays: {e:?}");
    }

    if let Err(e) = gtk::init() {
        panic!("failed to init gtk: {e:?}");
    }

    // Prepare icon
    let icon_theme_tmp_dir = get_icon_theme_temp_dir().unwrap();

    // Tray
    let mut indicator = AppIndicator::new("DDC Control Tray", "");
    indicator.set_status(AppIndicatorStatus::Active);
    indicator.set_icon_theme_path(icon_theme_tmp_dir.path().to_str().unwrap());
    indicator.set_icon_full("icon", "icon");

    // Build tray menu
    let mut menu = gtk::Menu::new();

    // Brightness shortcuts
    for bri in (0..101).step_by(10) {
        let mi = gtk::CheckMenuItem::with_label(format!("{}%", bri).as_str());
        mi.connect_activate(move |_| {
            set_brightness_all(bri);
        });
        menu.append(&mi);
    }

    // Divider
    let di = gtk::SeparatorMenuItem::new();
    menu.append(&di);

    // Reload displays
    let mi = gtk::CheckMenuItem::with_label("Reload displays");
    mi.connect_activate(|_| {
        reload_displays().unwrap();
    });
    menu.append(&mi);

    // Quit
    let mi = gtk::CheckMenuItem::with_label("Quit");
    mi.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.append(&mi);

    indicator.set_menu(&mut menu);
    menu.show_all();

    gtk::main();
}

fn set_brightness_all(brighness: u16) {
    let mut displays = DISPLAYS.lock().unwrap();

    for display in displays.iter_mut() {
        let mut retries = 0;

        loop {
            if retries >= SET_MAX_RETRIES {
                break;
            }

            if let Err(err) = set_brightness(brighness, display) {
                if err.to_string().contains("Remote I/O error") {
                    break;
                }
                if err.to_string().contains("invalid DDC/CI") {
                    break;
                }

                println!(
                    "failed to set value for display {:?} / {:?}: {:?}",
                    display.info.model_name, display.info.model_id, err
                );

                retries += 1;
                continue;
            }

            break;
        }
    }
}

fn set_brightness(brighness: u16, display: &mut Display) -> anyhow::Result<()> {
    let vcp_value = display.handle.get_vcp_feature(0x10)?;
    let vcp_value_max = vcp_value.maximum();
    let final_brighness = brighness * 100 / vcp_value_max;

    println!("Brightness final value to {:?}", final_brighness);

    display.handle.set_vcp_feature(0x10, final_brighness)?;

    // Ensure value changed.
    let vcp_value = display.handle.get_vcp_feature(0x10)?;
    if vcp_value.value() != final_brighness {
        return Err(anyhow!("invalid result"));
    }

    Ok(())
}

fn get_icon_theme_temp_dir() -> anyhow::Result<TempDir> {
    let icon_bytes = include_bytes!("icon.png");
    let temp_dir = TempDir::new(BIN_NAME)?;
    let icon_path = temp_dir.path().join("icon.png");
    let mut icon_file = File::create(icon_path)?;
    icon_file.write_all(icon_bytes)?;
    icon_file.sync_all()?;

    Ok(temp_dir)
}

fn reload_displays() -> anyhow::Result<()> {
    let current_displays = Display::enumerate();

    let mut displays = DISPLAYS.lock().unwrap();
    displays.clear();
    for dis in current_displays {
        displays.push(dis);
    }

    Ok(())
}
