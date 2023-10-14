use ddc_hi::{Ddc, Display};
use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

enum Message {
    Brightness { val: u16 },
    Quit,
}

fn main() {
    // Tray
    let mut tray =
        TrayItem::new("DDC control tray", IconSource::Resource("video-display")).unwrap();

    // Channel
    let (tx, rx) = mpsc::sync_channel::<Message>(2);

    // Brightness shortcuts
    for bri in (0..101).step_by(10) {
        let brigh_tx = tx.clone();

        tray.add_menu_item(format!("{}%", bri).as_str(), move || {
            brigh_tx.send(Message::Brightness { val: bri }).unwrap();
        })
        .unwrap();
    }

    // Quit
    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::Brightness { val }) => {
                println!("Brightness to {:?}", val);
                set_brightness(val);
            }
            _ => {}
        }
    }
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
