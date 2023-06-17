use crate::use_store::get_store;


pub fn game_exists(app_handle: tauri::AppHandle, id: &String) -> bool{
    get_store(app_handle.clone(), "game".to_string()+id).is_null()
}
