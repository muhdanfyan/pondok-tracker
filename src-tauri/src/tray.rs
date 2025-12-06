use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    App, Manager, Runtime,
};

pub fn setup_tray<R: Runtime>(app: &App<R>) -> Result<(), Box<dyn std::error::Error>> {
    let quit = MenuItem::with_id(app, "quit", "Keluar", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "Buka Dashboard", true, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[&show, &quit])?;
    
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Pondok Tracker")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        .build(app)?;
    
    Ok(())
}
