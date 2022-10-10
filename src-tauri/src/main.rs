use std::{
    process::Command,
    sync::{Arc, Mutex},
};

use block::ConcreteBlock;
use cocoa::{
    base::{id, nil},
    foundation::NSString,
};
use objc::{class, msg_send, sel, sel_impl};
use tauri::{
    AppHandle, CustomMenuItem, Manager, PhysicalPosition, PhysicalSize, State, SystemTray,
    SystemTrayEvent, SystemTrayMenu, Window,
};

#[derive(Clone, serde::Serialize)]
struct Event {
    message: String,
}

struct Id {
    id: id,
}
unsafe impl Send for Id {}

struct EventHandlers {
    handlers: Mutex<Vec<Id>>,
}

unsafe fn attach_spotify_event_listener(listener: Arc<dyn Fn()>) -> id {
    let notification_center: id = msg_send![class!(NSDistributedNotificationCenter), defaultCenter];

    let handler = ConcreteBlock::new({
        let listener = listener.clone();
        move || {
            (listener)();
        }
    })
    .copy();

    let observer_id: id = msg_send![notification_center, addObserverForName: ns_string("com.spotify.client.PlaybackStateChanged") object: nil queue: nil usingBlock: handler];
    observer_id
}

#[tauri::command]
fn init_spotify_event_emitter(app: AppHandle, handlers: State<EventHandlers>) {
    let handler = Arc::new(move || {
        app.emit_all(
            "playback-state-changed",
            Event {
                message: "playback_state_changed".into(),
            },
        )
        .unwrap();
    });
    let obs_id: id = unsafe { attach_spotify_event_listener(handler) };
    handlers.handlers.lock().unwrap().push(Id { id: obs_id });
}

#[tauri::command]
fn remove_spotify_event_observers(handlers: State<EventHandlers>) {
    let notification_center: id =
        unsafe { msg_send![class!(NSDistributedNotificationCenter), defaultCenter] };
    let mut handler_ids = handlers.handlers.lock().unwrap();
    for handler_id in handler_ids.iter() {
        let _: () = unsafe {
            msg_send![notification_center, removeObserver: handler_id.id name: ns_string("com.spotify.client.PlaybackStateChanged") object: nil]
        };
    }
    handler_ids.clear();
}

#[tauri::command]
fn get_current_track_info(keys: String) -> String {
    let cmd_str = format!("Tell application \"Spotify\" to get {{{keys}}} of current track");

    let output = Command::new("osascript")
        .arg("-e")
        .arg(cmd_str)
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stdout).unwrap()
}

#[tauri::command]
fn get_player_state(keys: String) -> String {
    let cmd_str = format!("Tell application \"Spotify\" to get {{{keys}}}");

    let output = Command::new("osascript")
        .arg("-e")
        .arg(cmd_str)
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stdout).unwrap()
}

#[tauri::command]
fn send_spotify_command(command: String) {
    let cmd_str = format!("Tell application \"Spotify\" to {command}");

    Command::new("osascript")
        .arg("-e")
        .arg(cmd_str)
        .spawn()
        .expect("Failed to execute command");
}

fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            let item_handle = app.tray_handle().get_item("show_hide");
            item_handle
                .set_title(if window.is_visible().unwrap() {
                    "Hide"
                } else {
                    "Show"
                })
                .unwrap();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show_hide" => {
                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                        item_handle.set_title("Show").unwrap();
                    } else {
                        window.show().unwrap();
                        item_handle.set_title("Hide").unwrap();
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn place_window(window: Window) {
    let screen = window.current_monitor().unwrap().unwrap();
    let screen_position = screen.position();

    let screen_size = PhysicalSize::<i32> {
        width: screen.size().width as i32,
        height: screen.size().height as i32,
    };
    let window_size = PhysicalSize::<i32> {
        width: window.outer_size().unwrap().width as i32,
        height: window.outer_size().unwrap().height as i32,
    };

    let top_right = PhysicalPosition {
        x: screen_position.x + (screen_size.width - window_size.width) - 50,
        y: screen_position.y + 100,
    };

    window
        .set_position(tauri::Position::Physical(top_right))
        .expect("unable to place window");
}

fn main() {
    let show_hide = CustomMenuItem::new("show_hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Mini Player");
    let tray_menu = SystemTrayMenu::new().add_item(show_hide).add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            place_window(app.get_window("main").unwrap());
            Ok(())
        })
        .manage(EventHandlers {
            handlers: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            init_spotify_event_emitter,
            remove_spotify_event_observers,
            get_current_track_info,
            get_player_state,
            send_spotify_command
        ])
        .system_tray(tray)
        .on_system_tray_event(|app, event| handle_system_tray_event(app, event))
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

unsafe fn ns_string(value: &str) -> id {
    NSString::alloc(nil).init_str(value)
}
