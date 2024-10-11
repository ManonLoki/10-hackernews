use anyhow::Result;
use tauri::{webview::PageLoadPayload, App, Webview, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent};
use tauri_plugin_log::Target;
use tracing::{ debug, info};
use utils::log_dir;

mod commands;
mod utils;

/// 2.0默认提供的入口点
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_app_dir
        ])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());
    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .title("Hacker News")
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }

    let webiview = builder.build()?;

    #[cfg(debug_assertions)]
    webiview.open_devtools();

    Ok(())
}

fn page_load_handler(webview:&Webview,_payload:&PageLoadPayload<'_>){
    info!("Page loaded on {:?}",webview.label());
}

fn window_event_handler(window:&Window,event:&WindowEvent){
    debug!("Window event {:?} on {:?}",event,window.label());

    if let WindowEvent::CloseRequested { api, .. }=event{
        info!("Close requested on {:?}",window.label());

        if window.label() == "main"{
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(tauri_plugin_log::TargetKind::Webview),
            Target::new(tauri_plugin_log::TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
            Target::new(tauri_plugin_log::TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}
