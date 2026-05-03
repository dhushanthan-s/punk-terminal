use crate::input::KeyEvent;
use crate::input::keyboard;
use crate::input::watch;
use serde_yaml::{Mapping, Value};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::sync::Mutex;
use std::sync::Once;

lazy_static::lazy_static! {
    static ref FUNCTION_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn add_or_update_binding() {
    check_if_init();
    println!("Press key combination to bind (e.g. ctrl+shift+d)");
    let mut captured_key_name: String = String::new();
    while captured_key_name.is_empty() {
        let key_event: KeyEvent = watch();
        captured_key_name = keyboard::key_fn_name_mapper(key_event);
    }
    println!("Captured {:?}", captured_key_name);
    print!("Action to perform : ");
    let mut action = String::new();
    std::io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");
    FUNCTION_MAP
        .lock()
        .unwrap()
        .insert(captured_key_name.to_string(), action.trim().to_string());
    write_into_yml(super::path_of_file("user_keybinding.yml".to_string()));
    println!("Captured {} and mapped with {}", captured_key_name, action);
}

/// Look up action for keybind string. Tries canonical form (ctrl+c) then legacy (ctrl-c).
fn lookup_action(key_name: &str) -> Option<String> {
    let map = FUNCTION_MAP.lock().unwrap();
    map.get(key_name).cloned().or_else(|| {
        let legacy: String = key_name.replace('+', "-");
        map.get(&legacy).cloned()
    })
}

pub fn handle_and_call(key_event: KeyEvent) {
    check_if_init();
    let key_name: String = keyboard::key_fn_name_mapper(key_event.clone());
    let func_to_call = if key_name.is_empty() {
        pass as fn()
    } else if let Some(func) = lookup_action(&key_name) {
        function_name_mapper(func)
    } else {
        println!("Function not found for key {:?}", key_name);
        pass as fn()
    };
    func_to_call();
}

fn check_if_init() {
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        key_binder_init();
    });
}

fn key_binder_init() {
    let default_file_path: String = super::path_of_file("default_keybinding.yml".to_string());
    let default_file = File::open(default_file_path.as_str()).unwrap();
    let default_reader = BufReader::new(default_file);
    let default_binding_map: HashMap<String, String> =
        serde_yaml::from_reader(default_reader).unwrap();

    let user_file_path: String = super::path_to_user_conf("user_keybinding.yml".to_string());
    let user_file = File::open(user_file_path.as_str());
    match user_file {
        Ok(user_file) => {
            let user_reader = BufReader::new(user_file);
            let user_binding_map: HashMap<String, String> =
                serde_yaml::from_reader::<_, HashMap<String, String>>(user_reader).unwrap();

            for (key, value) in &user_binding_map {
                FUNCTION_MAP
                    .lock()
                    .unwrap()
                    .insert(key.to_string(), value.to_string());
            }
            write_into_yml(user_file_path);
        }

        Err(_user_file) => {
            // Do nothing
        }
    }

    for (key, value) in &default_binding_map {
        FUNCTION_MAP
            .lock()
            .unwrap()
            .insert(key.to_string(), value.to_string());
    }
}

fn function_name_mapper(name: String) -> fn() {
    match name.as_str() {
        "clear" => clear as fn(),
        "exit" => exit as fn(),
        "add_or_update_binding" => add_or_update_binding as fn(),
        _ => pass as fn(),
    }
}

fn write_into_yml(filename: String) {
    let mut mapping = Mapping::new();
    for (key, value) in FUNCTION_MAP.lock().unwrap().clone() {
        mapping.insert(Value::String(key), Value::String(value));
    }
    let yaml_value = Value::Mapping(mapping);
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(filename.clone())
    {
        Ok(f) => f,
        Err(_) => File::create(filename.clone()).expect("Failed to create file"),
    };
    serde_yaml::to_writer(&mut file, &yaml_value).expect("Failed to write YAML");
}

fn clear() {
    println!("clearing....");
}

fn exit() {
    std::process::exit(0);
}

fn pass() {}
