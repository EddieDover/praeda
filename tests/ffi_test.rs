#![allow(unsafe_code, unused_unsafe)]

use praeda::ffi::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Helper function to convert C string to Rust string
fn c_str_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}

#[test]
fn test_ffi_generator_new_and_free() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());
        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_generator_version() {
    unsafe {
        let version_ptr = praeda_version();
        assert!(!version_ptr.is_null());

        let version = c_str_to_string(version_ptr);
        assert!(!version.is_empty());
        assert!(version.contains("."));

        praeda_string_free(version_ptr as *mut c_char);
    }
}

#[test]
fn test_ffi_load_toml() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        let toml_str = r#"
[quality_data]
common = 100
uncommon = 60
rare = 30

[[item_types]]
item_type = "weapon"
weight = 1
[item_types.subtypes]
sword = 1

[[item_attributes]]
item_type = "weapon"
subtype = ""
[[item_attributes.attributes]]
name = "damage"
initial_value = 10.0
min = 1.0
max = 20.0
required = true

[[item_list]]
item_type = "weapon"
subtype = "sword"
names = ["longsword", "shortsword"]
"#;

        let toml_c_str = CString::new(toml_str).unwrap();
        let result_ptr = praeda_generator_load_toml(handle, toml_c_str.as_ptr());

        assert!(!result_ptr.is_null());

        let result_str = c_str_to_string(result_ptr);
        assert!(result_str.contains("success"));
        assert!(result_str.contains("true"));

        praeda_string_free(result_ptr as *mut c_char);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_load_toml_invalid_handle() {
    unsafe {
        let invalid_handle: *mut PraedaGeneratorHandle = std::ptr::null_mut();
        let toml_str = "[quality_data]\ncommon = 100";
        let toml_c_str = CString::new(toml_str).unwrap();

        let result_ptr = praeda_generator_load_toml(invalid_handle, toml_c_str.as_ptr());

        assert!(!result_ptr.is_null());

        let result_str = c_str_to_string(result_ptr);
        assert!(result_str.contains("success"));
        assert!(result_str.contains("false"));
        assert!(result_str.contains("error"));

        praeda_string_free(result_ptr as *mut c_char);
    }
}

#[test]
fn test_ffi_generate_loot() {
    unsafe {
        let handle = praeda_generator_new();

        // First load configuration
        let toml_str = r#"
[quality_data]
common = 100
uncommon = 60

[[item_types]]
item_type = "weapon"
weight = 1
[item_types.subtypes]
sword = 1

[[item_attributes]]
item_type = "weapon"
subtype = ""
[[item_attributes.attributes]]
name = "damage"
initial_value = 10.0
min = 1.0
max = 20.0
required = true

[[item_list]]
item_type = "weapon"
subtype = "sword"
names = ["longsword"]
"#;

        let toml_c_str = CString::new(toml_str).unwrap();
        let load_result_ptr = praeda_generator_load_toml(handle, toml_c_str.as_ptr());
        praeda_string_free(load_result_ptr as *mut c_char);

        // Now generate loot
        let options_json = r#"{
            "number_of_items": 5,
            "base_level": 10.0,
            "level_variance": 2.0,
            "affix_chance": 0.25,
            "linear": true,
            "scaling_factor": 1.0
        }"#;

        let options_c_str = CString::new(options_json).unwrap();
        let loot_ptr = praeda_generator_generate_loot(handle, options_c_str.as_ptr());

        assert!(!loot_ptr.is_null());

        let loot_str = c_str_to_string(loot_ptr);
        assert!(!loot_str.is_empty());
        assert!(loot_str.contains("["));
        assert!(loot_str.contains("]"));

        // Should be valid JSON array
        let _: Vec<serde_json::Value> = serde_json::from_str(&loot_str)
            .expect("loot JSON should be valid");

        praeda_string_free(loot_ptr as *mut c_char);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_generate_loot_invalid_json() {
    unsafe {
        let handle = praeda_generator_new();

        let toml_str = r#"
[quality_data]
common = 100

[[item_types]]
item_type = "weapon"
weight = 1
[item_types.subtypes]
sword = 1

[[item_attributes]]
item_type = "weapon"
subtype = ""
[[item_attributes.attributes]]
name = "damage"
initial_value = 10.0
min = 1.0
max = 20.0
required = true

[[item_list]]
item_type = "weapon"
subtype = "sword"
names = ["sword"]
"#;

        let toml_c_str = CString::new(toml_str).unwrap();
        praeda_generator_load_toml(handle, toml_c_str.as_ptr());

        // Invalid JSON
        let invalid_json = "{ not valid json }";
        let invalid_c_str = CString::new(invalid_json).unwrap();

        let result_ptr = praeda_generator_generate_loot(handle, invalid_c_str.as_ptr());

        assert!(!result_ptr.is_null());

        let result_str = c_str_to_string(result_ptr);
        assert!(result_str.contains("error") || result_str.contains("success\": false"));

        praeda_string_free(result_ptr as *mut c_char);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_get_quality_data() {
    unsafe {
        let handle = praeda_generator_new();

        let toml_str = r#"
[quality_data]
common = 100
rare = 30
"#;

        let toml_c_str = CString::new(toml_str).unwrap();
        praeda_generator_load_toml(handle, toml_c_str.as_ptr());

        let quality_ptr = praeda_generator_get_quality_data(handle);

        assert!(!quality_ptr.is_null());

        let quality_str = c_str_to_string(quality_ptr);
        assert!(quality_str.contains("common"));
        assert!(quality_str.contains("rare"));

        let _: serde_json::Value = serde_json::from_str(&quality_str)
            .expect("quality data should be valid JSON");

        praeda_string_free(quality_ptr as *mut c_char);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_has_quality() {
    unsafe {
        let handle = praeda_generator_new();

        let toml_str = r#"
[quality_data]
common = 100
rare = 30
"#;

        let toml_c_str = CString::new(toml_str).unwrap();
        praeda_generator_load_toml(handle, toml_c_str.as_ptr());

        let common_c_str = CString::new("common").unwrap();
        let result = praeda_generator_has_quality(handle, common_c_str.as_ptr());
        assert_eq!(result, 1);

        let rare_c_str = CString::new("rare").unwrap();
        let result = praeda_generator_has_quality(handle, rare_c_str.as_ptr());
        assert_eq!(result, 1);

        let nonexistent_c_str = CString::new("nonexistent").unwrap();
        let result = praeda_generator_has_quality(handle, nonexistent_c_str.as_ptr());
        assert_eq!(result, 0);

        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_has_quality_invalid_handle() {
    unsafe {
        let invalid_handle: *mut PraedaGeneratorHandle = std::ptr::null_mut();
        let quality_c_str = CString::new("common").unwrap();

        let result = praeda_generator_has_quality(invalid_handle, quality_c_str.as_ptr());

        assert_eq!(result, -1); // Error indicator
    }
}

#[test]
fn test_ffi_generator_info() {
    unsafe {
        let handle = praeda_generator_new();

        let toml_str = r#"
[quality_data]
common = 100
rare = 30

[[item_types]]
item_type = "weapon"
weight = 1
[item_types.subtypes]
sword = 1

[[item_types]]
item_type = "armor"
weight = 1
[item_types.subtypes]
chest = 1
"#;

        let toml_c_str = CString::new(toml_str).unwrap();
        praeda_generator_load_toml(handle, toml_c_str.as_ptr());

        let info_ptr = praeda_generator_info(handle);

        assert!(!info_ptr.is_null());

        let info_str = c_str_to_string(info_ptr);
        assert!(info_str.contains("version"));
        assert!(info_str.contains("qualities"));
        assert!(info_str.contains("item_types"));

        let info: serde_json::Value = serde_json::from_str(&info_str)
            .expect("info should be valid JSON");

        assert!(info["version"].is_string());
        assert!(info["qualities"].is_number());
        assert!(info["item_types"].is_number());

        // Verify counts are correct
        assert_eq!(info["qualities"].as_u64().unwrap(), 2);
        assert_eq!(info["item_types"].as_u64().unwrap(), 2);

        praeda_string_free(info_ptr as *mut c_char);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_generator_info_invalid_handle() {
    unsafe {
        let invalid_handle: *mut PraedaGeneratorHandle = std::ptr::null_mut();
        let info_ptr = praeda_generator_info(invalid_handle);

        assert!(info_ptr.is_null());
    }
}

#[test]
fn test_ffi_string_free_null() {
    // Should not panic when freeing null pointer
    unsafe {
        praeda_string_free(std::ptr::null_mut());
    }
}

#[test]
fn test_ffi_programmatic_configuration() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Configure via programmatic API instead of TOML
        // Set qualities
        let result = praeda_generator_set_quality_data(
            handle,
            CString::new("common").unwrap().as_ptr(),
            100,
        );
        assert!(!result.is_null());
        praeda_string_free(result as *mut c_char);

        let result = praeda_generator_set_quality_data(
            handle,
            CString::new("rare").unwrap().as_ptr(),
            30,
        );
        assert!(!result.is_null());
        praeda_string_free(result as *mut c_char);

        // Set item type
        let result = praeda_generator_set_item_type(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            1,
        );
        assert!(!result.is_null());
        praeda_string_free(result as *mut c_char);

        // Set item subtype
        let result = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            1,
        );
        assert!(!result.is_null());
        praeda_string_free(result as *mut c_char);

        // Set attribute
        let result = praeda_generator_set_attribute(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("").unwrap().as_ptr(),
            CString::new("damage").unwrap().as_ptr(),
            10.0,
            1.0,
            20.0,
            1,
        );
        assert!(!result.is_null());
        praeda_string_free(result as *mut c_char);

        // Set item names
        let names_json = CString::new("[\"longsword\", \"shortsword\"]").unwrap();
        let result = praeda_generator_set_item_names(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            names_json.as_ptr(),
        );
        assert!(!result.is_null());
        praeda_string_free(result as *mut c_char);

        // Now generate loot with programmatically configured generator
        let options_json = r#"{
            "number_of_items": 5,
            "base_level": 10.0,
            "level_variance": 2.0,
            "affix_chance": 0.25,
            "linear": true,
            "scaling_factor": 1.0
        }"#;

        let options_c_str = CString::new(options_json).unwrap();
        let loot_ptr = praeda_generator_generate_loot(handle, options_c_str.as_ptr());

        assert!(!loot_ptr.is_null());

        let loot_str = c_str_to_string(loot_ptr);
        assert!(!loot_str.is_empty());

        let items: Vec<serde_json::Value> = serde_json::from_str(&loot_str)
            .expect("loot should be valid JSON");

        assert_eq!(items.len(), 5);

        praeda_string_free(loot_ptr as *mut c_char);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_ffi_multiple_operations_sequence() {
    unsafe {
        let handle = praeda_generator_new();

        // Load TOML
        let toml_str = r#"
[quality_data]
common = 100
uncommon = 60
rare = 30

[[item_types]]
item_type = "weapon"
weight = 2
[item_types.subtypes]
sword = 1
axe = 1

[[item_types]]
item_type = "armor"
weight = 1
[item_types.subtypes]
chest = 1

[[item_attributes]]
item_type = "weapon"
subtype = ""
[[item_attributes.attributes]]
name = "damage"
initial_value = 15.0
min = 5.0
max = 30.0
required = true

[[item_attributes]]
item_type = "armor"
subtype = ""
[[item_attributes.attributes]]
name = "defense"
initial_value = 10.0
min = 2.0
max = 20.0
required = true

[[item_list]]
item_type = "weapon"
subtype = "sword"
names = ["longsword", "shortsword"]

[[item_list]]
item_type = "weapon"
subtype = "axe"
names = ["battleaxe"]

[[item_list]]
item_type = "armor"
subtype = "chest"
names = ["plate", "leather"]
"#;

        let toml_c_str = CString::new(toml_str).unwrap();
        let load_result = praeda_generator_load_toml(handle, toml_c_str.as_ptr());
        assert!(!load_result.is_null());
        praeda_string_free(load_result as *mut c_char);

        // Get quality data
        let quality_ptr = praeda_generator_get_quality_data(handle);
        assert!(!quality_ptr.is_null());
        praeda_string_free(quality_ptr as *mut c_char);

        // Check qualities
        let common_c_str = CString::new("common").unwrap();
        let has_common = praeda_generator_has_quality(handle, common_c_str.as_ptr());
        assert_eq!(has_common, 1);

        // Generate loot multiple times
        for i in 0..3 {
            let options_json = format!(
                r#"{{
                    "number_of_items": {},
                    "base_level": 10.0,
                    "level_variance": 5.0,
                    "affix_chance": 0.25,
                    "linear": true,
                    "scaling_factor": 1.0
                }}"#,
                1 + i
            );

            let options_c_str = CString::new(options_json).unwrap();
            let loot_ptr = praeda_generator_generate_loot(handle, options_c_str.as_ptr());

            assert!(!loot_ptr.is_null());

            let loot_str = c_str_to_string(loot_ptr);
            let items: Vec<serde_json::Value> = serde_json::from_str(&loot_str)
                .expect("loot should be valid JSON");

            assert_eq!(items.len(), 1 + i);

            praeda_string_free(loot_ptr as *mut c_char);
        }

        // Get info
        let info_ptr = praeda_generator_info(handle);
        assert!(!info_ptr.is_null());
        praeda_string_free(info_ptr as *mut c_char);

        // Cleanup
        praeda_generator_free(handle);
    }
}
