use anyhow::anyhow;
use lazy_static::lazy_static;
use shared::*;
use std::sync::Arc;
use tauri::Emitter;

mod http;

lazy_static! {
    static ref CORE: Arc<Core<CrabNews>> = Arc::new(Core::new());
}

fn handle_event(
    event: Event,
    core: &Arc<Core<CrabNews>>,
    tauri_app: tauri::AppHandle,
) -> anyhow::Result<()> {
    for effect in core.process_event(event) {
        process_effect(effect, core, tauri_app.clone())?
    }

    Ok(())
}

fn process_effect(
    effect: Effect,
    core: &Arc<Core<CrabNews>>,
    tauri_app: tauri::AppHandle,
) -> anyhow::Result<()> {
    match effect {
        Effect::Render(_) => {
            let view = core.view();
            tauri_app.emit("render", view).map_err(|e| anyhow!(e))
        }
        Effect::Http(mut request) => {
            tauri::async_runtime::spawn({
                let core = core.clone();

                async move {
                    let response = http::request(&request.operation).await;
                    for effect in core
                        .resolve(&mut request, response.into())
                        .map_err(|e| anyhow!(e))?
                    {
                        process_effect(effect, &core, tauri_app.clone())?;
                    }

                    anyhow::Ok(())
                }
            });

            Ok(())
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn create_account(app_handle: tauri::AppHandle, account_type: AccountType) {
    let _ = handle_event(Event::CreateAccount(account_type), &CORE, app_handle);
}

#[tauri::command]
async fn delete_account(app_handle: tauri::AppHandle, account: Account) {
    let _ = handle_event(Event::DeleteAccount(account), &CORE, app_handle);
}

#[tauri::command]
async fn rename_account(
    app_handle: tauri::AppHandle,
    old_account_name: OldAccountName,
    new_account_name: NewAccountName,
) {
    let _ = handle_event(
        Event::RenameAccount(old_account_name, new_account_name),
        &CORE,
        app_handle,
    );
}

#[tauri::command]
async fn get_feed(app_handle: tauri::AppHandle, account: Account, sub_link: SubscriptionLink) {
    let _ = handle_event(Event::GetFeed(account, sub_link), &CORE, app_handle);
}

/// The main entry point for Tauri
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_account,
            delete_account,
            rename_account,
            get_feed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
