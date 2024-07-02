// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, KeyboardControllable};
use std::{process::exit, thread};

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use tao::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![keydown, keyup])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let event_loop = EventLoopBuilder::new().build();
    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);
    let hotkey2 = HotKey::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyD);
    let hotkey3 = HotKey::new(None, Code::KeyF);
    let quit = HotKey::new(Some(Modifiers::META), Code::KeyA);

    hotkeys_manager.register(hotkey).unwrap();
    hotkeys_manager.register(hotkey2).unwrap();
    hotkeys_manager.register(hotkey3).unwrap();
    hotkeys_manager.register(quit).unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = global_hotkey_channel.try_recv() {
            println!("{event:?}");

            if hotkey2.id() == event.id && event.state == HotKeyState::Released {
                hotkeys_manager.unregister(hotkey2).unwrap();
            }

            if event.id == quit.id() && event.state == HotKeyState::Pressed {
                exit(1);
            }
        }
    });
}

#[tauri::command]
fn keydown(key: char) {
    let mut enigo = Enigo::new();
    // enigo.mouse_move_to(500, 200);
    // enigo.mouse_click(enigo::MouseButton::Right);
    enigo.key_down(enigo::Key::Layout(key));
    println!("{}", key);
    // format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn keyup(key: char) {
    let mut enigo = Enigo::new();
    // enigo.mouse_move_to(500, 200);
    // enigo.mouse_click(enigo::MouseButton::Right);
    enigo.key_up(enigo::Key::Layout(key));
    println!("{}", key);
    // format!("Hello, {}! You've been greeted from Rust!", name)
}
