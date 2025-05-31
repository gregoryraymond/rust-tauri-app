use std::sync::OnceLock;

use anyhow::Result;
use tauri::AppHandle;
use tauri::{plugin::PermissionState, Window};
use tauri_plugin_blec::check_permissions;
use tauri_plugin_geolocation::{GeolocationExt, PermissionType, PositionOptions};
use tauri_plugin_shell::process::Command;
use tauri_plugin_shell::ShellExt;

use geo::{Contains, Coord, Geometry};
use geojson::{GeoJson, Value::Polygon};

fn load_file(file: String) -> Option<geo::Polygon> {
    if let GeoJson::FeatureCollection(home_geojson) = file.parse::<GeoJson>().unwrap() {
        for feature in home_geojson.features {
            if let Polygon(poly) = feature.geometry.unwrap().value {
                let exterior = poly.iter().map(|x| 
                    geo::LineString::new(x.iter().map(|y| Coord { x: y[0], y: y[1] }).collect::<Vec<Coord>>())
                ).collect::<Vec<geo::LineString>>();
                return Some(geo::Polygon::new(exterior[0].clone(), vec![]));
            }
        }
    }
    None
}

fn home() -> &'static Geometry {
    static HOME: OnceLock<Geometry> = OnceLock::new();
    HOME.get_or_init(|| {
        if let Some(home) = load_file(String::from(include_str!("../locations/home.geojson"))) {
            home.into()
        } else {
            panic!("Home not set.")
        }
    })
}

fn school() -> &'static Geometry {
    static SCHOOL: OnceLock<Geometry> = OnceLock::new();
    SCHOOL.get_or_init(|| {
        if let Some(school) = load_file(String::from(include_str!("../locations/school.geojson"))) {
            school.into()
        } else {
            panic!("School not set.")
        }
    })
}

fn work() -> &'static Geometry {
    static WORK: OnceLock<Geometry> = OnceLock::new();
    WORK.get_or_init(|| {
        if let Some(work) = load_file(String::from(include_str!("../locations/work.geojson"))) {
            work.into()
        } else {
            panic!("Work not set.")
        }
    })
}

#[tauri::command]
async fn connect_hk203(app: AppHandle) -> Result<(), String> {
    bluetooth_internal(app).await.map_err(|e| e.to_string())
}

async fn bluetooth_internal(app: AppHandle) -> anyhow::Result<()> {
    check_permissions()?;

    let handler = tauri_plugin_blec::get_handler()?;
    handler
        .connect("7A:75:F0:F5:31:F3", (|| println!("disconnected")).into())
        .await?;

    let cmd = app.shell();
    cmd.command("input keyevent 126");
    Ok(())
}

#[tauri::command]
async fn get_current_zone(
    position: tauri_plugin_geolocation::Position
) -> Result<String, String> {
    let cord = geo::coord! { x: position.coords.latitude, y: position.coords.longitude };
    Ok(if home().contains(&cord) {
        String::from("Home")
    } else if work().contains(&cord) {
        String::from("Work")
    } else if school().contains(&cord) {
        String::from("School")
    } else {
        String::from("In-Transit")
    })
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_current_position(
    window: Window,
) -> Result<tauri_plugin_geolocation::Position, String> {
    get_position_internal(window)
        .await
        .map_err(|e| e.to_string())
}

async fn get_position_internal(
    window: Window,
) -> anyhow::Result<tauri_plugin_geolocation::Position> {
    let location = window.geolocation();
    let mut perm = location.check_permissions()?;
    loop {
        perm = match perm.location {
            PermissionState::Granted => break,
            PermissionState::Denied => continue,
            PermissionState::Prompt => {
                location.request_permissions(Some(vec![PermissionType::Location]))?
            }
            PermissionState::PromptWithRationale => {
                location.request_permissions(Some(vec![PermissionType::Location]))?
            }
        };
    }
    Ok(location.get_current_position(Some(PositionOptions {
        enable_high_accuracy: true,
        timeout: 10000,
        maximum_age: 0,
    }))?)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_geolocation::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_blec::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_current_position,
            connect_hk203,
            get_current_zone
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use geo::Contains;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test() -> &'static Geometry {
        static TEST: OnceLock<Geometry> = OnceLock::new();
        TEST.get_or_init(|| {
            if let Some(test) = load_file(String::from(include_str!("../locations/test.geojson"))) {
                test.into()
            } else {
                panic!("Test not set.")
            }
        })
    }

    #[test]
    fn test_add() {
        let val = test();
        let coord = geo::coord! { x: 148.60224773917474, y: -35.41486350377471 };
        assert!(val.contains(&coord));
    }
}