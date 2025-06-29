// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime, WebviewUrl, WebviewWindowBuilder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct IpApiResponse {
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
    country: Option<String>,
    status: String,
}

#[derive(Debug, Serialize)]
struct CountryInfo {
    code: String,
    name: String,
    flag_emoji: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn get_public_ip_info() -> Result<CountryInfo, String> {
    // Try multiple IP detection services
    let services = vec![
        "http://ip-api.com/json/?fields=status,country,countryCode",
        "https://ipapi.co/json/",
        "https://api.ipify.org?format=json",
    ];
    
    for service_url in services {
        match fetch_ip_info(service_url).await {
            Ok(info) => return Ok(info),
            Err(_) => continue,
        }
    }
    
    // If all services fail, return Earth icon info
    Ok(CountryInfo {
        code: "EARTH".to_string(),
        name: "Unknown".to_string(),
        flag_emoji: "🌍".to_string(),
    })
}

async fn fetch_ip_info(url: &str) -> Result<CountryInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    if url.contains("ip-api.com") {
        let response: IpApiResponse = client.get(url).send().await?.json().await?;
        
        if response.status == "success" {
            let country_code = response.country_code.unwrap_or("EARTH".to_string());
            let country_name = response.country.unwrap_or("Unknown".to_string());
            let flag_emoji = get_flag_emoji(&country_code);
            
            return Ok(CountryInfo {
                code: country_code,
                name: country_name,
                flag_emoji,
            });
        }
    }
    
    Err("Failed to get IP info".into())
}

fn get_flag_emoji(country_code: &str) -> String {
    match country_code {
        "US" => "🇺🇸".to_string(),
        "CN" => "🇨🇳".to_string(),
        "JP" => "🇯🇵".to_string(),
        "DE" => "🇩🇪".to_string(),
        "GB" => "🇬🇧".to_string(),
        "FR" => "🇫🇷".to_string(),
        "CA" => "🇨🇦".to_string(),
        "AU" => "🇦🇺".to_string(),
        "KR" => "🇰🇷".to_string(),
        "BR" => "🇧🇷".to_string(),
        "IN" => "🇮🇳".to_string(),
        "RU" => "🇷🇺".to_string(),
        "IT" => "🇮🇹".to_string(),
        "ES" => "🇪🇸".to_string(),
        "NL" => "🇳🇱".to_string(),
        "SE" => "🇸🇪".to_string(),
        "NO" => "🇳🇴".to_string(),
        "FI" => "🇫🇮".to_string(),
        "DK" => "🇩🇰".to_string(),
        "CH" => "🇨🇭".to_string(),
        "AT" => "🇦🇹".to_string(),
        "BE" => "🇧🇪".to_string(),
        "PL" => "🇵🇱".to_string(),
        "CZ" => "🇨🇿".to_string(),
        "HU" => "🇭🇺".to_string(),
        "PT" => "🇵🇹".to_string(),
        "GR" => "🇬🇷".to_string(),
        "TR" => "🇹🇷".to_string(),
        "IL" => "🇮🇱".to_string(),
        "AE" => "🇦🇪".to_string(),
        "SA" => "🇸🇦".to_string(),
        "EG" => "🇪🇬".to_string(),
        "ZA" => "🇿🇦".to_string(),
        "NG" => "🇳🇬".to_string(),
        "KE" => "🇰🇪".to_string(),
        "MX" => "🇲🇽".to_string(),
        "AR" => "🇦🇷".to_string(),
        "CL" => "🇨🇱".to_string(),
        "CO" => "🇨🇴".to_string(),
        "PE" => "🇵🇪".to_string(),
        "VE" => "🇻🇪".to_string(),
        "TH" => "🇹🇭".to_string(),
        "VN" => "🇻🇳".to_string(),
        "MY" => "🇲🇾".to_string(),
        "SG" => "🇸🇬".to_string(),
        "ID" => "🇮🇩".to_string(),
        "PH" => "🇵🇭".to_string(),
        "NZ" => "🇳🇿".to_string(),
        "TW" => "🇹🇼".to_string(),
        "HK" => "🇭🇰".to_string(),
        _ => "🌍".to_string(), // Earth emoji for unknown countries
    }
}

fn setup_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let _tray = TrayIconBuilder::with_id("main-tray")
        .tooltip("IPStatus - Checking location...")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: tauri::tray::MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    let _window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                        .title("IPStatus")
                        .build();
                }
            }
        })
        .build(app)?;

    // Fetch IP info and update tray icon
    let app_handle = app.clone();
    tokio::spawn(async move {
        match get_public_ip_info().await {
            Ok(country_info) => {
                let tooltip = format!("IPStatus - {} {}", country_info.flag_emoji, country_info.name);
                if let Some(tray) = app_handle.tray_by_id("main-tray") {
                    let _ = tray.set_tooltip(Some(tooltip));
                }
            }
            Err(_) => {
                if let Some(tray) = app_handle.tray_by_id("main-tray") {
                    let _ = tray.set_tooltip(Some("IPStatus - 🌍 Unknown Location"));
                }
            }
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![get_public_ip_info])
        .setup(|app| {
            setup_tray(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
