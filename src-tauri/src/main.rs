// Mars Swap: a small native window around the swap engine.
//
// The shell does three things and nothing else:
//   1. generate a per-launch session token,
//   2. start the bundled engine (atomicswapd --desktop) with that token,
//   3. open the window at the engine's local address once it is ready.
// Everything to do with keys and swaps happens inside the engine.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rand::RngCore;
use tauri::{Manager, RunEvent, Url};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};
use tauri_plugin_shell::ShellExt;

/// The running engine. Dropping it closes the engine's stdin, which the
/// engine takes as the signal to save and exit.
struct Engine(Mutex<Option<CommandChild>>);

fn session_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let token = session_token();
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;

            // The token goes by environment, never on the command line,
            // so other processes can't read it from the process list.
            let (mut events, child) = app
                .shell()
                .sidecar("atomicswapd")?
                .args(["--desktop", "--port", "0"])
                .env("ATOMICSWAPD_DATA", data_dir.to_string_lossy().to_string())
                .env("ATOMICSWAPD_SESSION_TOKEN", token.clone())
                .spawn()?;
            app.manage(Engine(Mutex::new(Some(child))));

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Some(event) = events.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            let line = String::from_utf8_lossy(&line);
                            if let Some(base) = line.trim().strip_prefix("ATOMICSWAPD_READY ") {
                                // The fragment is never sent over HTTP; the page
                                // trades it for a session cookie and removes it.
                                let target = format!("{}/#token={token}", base.trim_end_matches('/'));
                                if let (Some(window), Ok(url)) =
                                    (handle.get_webview_window("main"), Url::parse(&target))
                                {
                                    let _ = window.navigate(url);
                                }
                            }
                        }
                        CommandEvent::Terminated(status) => {
                            if let Some(window) = handle.get_webview_window("main") {
                                let code = status.code.map(|c| c.to_string()).unwrap_or_else(|| "none".into());
                                let _ = window.eval(&format!(
                                    "window.engineStopped && window.engineStopped('{code}')"
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Mars Swap");

    app.run(|handle, event| {
        if let RunEvent::Exit = event {
            // Close the engine's stdin; it saves the wallet and exits by itself.
            if let Some(engine) = handle.try_state::<Engine>() {
                drop(engine.0.lock().ok().and_then(|mut child| child.take()));
            }
        }
    });
}
