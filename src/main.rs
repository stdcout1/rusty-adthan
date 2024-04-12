// use std::time::Duration;
// use std::thread;
//
// use rusty_adthan::{PrayerResults, Prayers};
//
// fn main() {
//     let mut prayers = Prayers::new("Toronto".to_string(), "Canada".to_string()).unwrap();
//
//     loop {
//         // this should catch unexpected wakeups on unix systems
//         match prayers.get_next_prayer_unix(3).unwrap() {
//             PrayerResults::Prayer(prayer) => {
//                 println!("it is {} time", prayer.name);
//             }
//             PrayerResults::CaughtUp => {
//                 println!("I am skipping")
//             }
//             PrayerResults::NotTimeYet(dur) => {
//                 println!("{}", dur);
//                 thread::sleep(Duration::from_secs(dur as u64))
//             }
//         }
//         thread::sleep(Duration::from_secs(1))
//     }
// }



// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![allow(unused)]

use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");

    // Since winit doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;

        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .build()
            .unwrap();

        gtk::main();
    });

    let event_loop = EventLoopBuilder::new().build().unwrap();

    #[cfg(not(target_os = "linux"))]
    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(Menu::new()))
            .with_tooltip("winit - awesome windowing lib")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Poll);

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
    });
}
