use tauri::Manager;
use tauri_plugin_store::with_store;
use serde_json::Value as JsonValue;



pub fn set_store(app_handle: tauri::AppHandle, key: String, value: JsonValue){
    let path = app_handle.path_resolver().app_data_dir().unwrap().join(".settings.dat");
    with_store(app_handle.clone(), app_handle.state(), path, |store| {
        store.insert(key.into(), value).unwrap();
        store.save().unwrap();
        Ok(())
    }).unwrap();
}

 pub fn get_store(app_handle: tauri::AppHandle, key: String) -> JsonValue {
    let path = app_handle.path_resolver().app_data_dir().unwrap().join(".settings.dat");
    let mut data = JsonValue::Null;

    with_store(app_handle.clone(), app_handle.state(), path, |store| {
        data = store.get(key).unwrap().clone();
        Ok(())
    }).unwrap();
   
    return data;
}