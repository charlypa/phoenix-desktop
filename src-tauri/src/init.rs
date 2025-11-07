use crate::utilities::ensure_dir_exists;
use crate::boot_config::read_boot_config;
use crate::boot_config::APP_CONSTANTS;
use crate::boot_config::AppConstants;

pub fn init_app(app: &mut tauri::App) {
    let config = app.config().clone();
    let app_handle = app.handle();

    println!("Appdata path is {}", app_handle.path().app_local_data_dir().expect("failed to retrieve app_local_data_dir").display());
    ensure_dir_exists(&app_handle.path().app_local_data_dir().unwrap()); // canonicalize will work only if path exists
    let _ = APP_CONSTANTS.set(AppConstants {
        tauri_config: config.clone(),
        app_local_data_dir: app_handle.path().app_local_data_dir().expect("failed to retrieve app_local_data_dir")
            .canonicalize().expect("Failed to canonicalize app_local_data_dir")
    });

    // To get a value
    if let Some(app_constants) = APP_CONSTANTS.get() {
        #[cfg(debug_assertions)]{
            println!("Bundle ID is {}", app_constants.tauri_config.tauri.bundle.identifier);
        }
        ensure_dir_exists(&app_constants.app_local_data_dir);
        read_boot_config();
        #[cfg(debug_assertions)]{
            println!("Bootconfig version is {}", read_boot_config().version);
        }
    }
}