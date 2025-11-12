//! Foreign Function Interface (FFI) for Praeda
//!
//! This module provides C-compatible bindings for the Praeda loot generation library.
//! It allows usage from C++, C#, and other languages that can call C functions.
//!
//! All data is exchanged through JSON strings for simplicity and language independence.

#![allow(clippy::not_unsafe_ptr_arg_deref)]

use crate::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Opaque pointer to a PraedaGenerator instance
///
/// This pointer should only be created via praeda_generator_new()
/// and freed via praeda_generator_free()
pub struct PraedaGeneratorHandle {
    generator: Box<PraedaGenerator>,
}

// ============================================================================
// Memory Management
// ============================================================================

/// Create a new Praeda generator
///
/// # Returns
/// A pointer to a new PraedaGenerator instance. Must be freed with praeda_generator_free().
/// Returns null pointer on failure.
///
/// # Safety
/// The returned pointer must be freed with praeda_generator_free() to avoid memory leaks.
#[no_mangle]
pub extern "C" fn praeda_generator_new() -> *mut PraedaGeneratorHandle {
    Box::into_raw(Box::new(PraedaGeneratorHandle {
        generator: Box::new(PraedaGenerator::new()),
    }))
}

/// Free a Praeda generator instance
///
/// # Arguments
/// * `handle` - Pointer to generator created by praeda_generator_new()
///
/// # Safety
/// The pointer must be valid and must have been created by praeda_generator_new().
/// After calling this function, the pointer must not be used again.
#[no_mangle]
pub extern "C" fn praeda_generator_free(handle: *mut PraedaGeneratorHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

// ============================================================================
// TOML Configuration
// ============================================================================

/// Load configuration from a TOML string
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `toml_str` - TOML configuration as null-terminated C string
///
/// # Returns
/// JSON object with status. Example:
/// ```json
/// {"success": true}
/// ```
/// or
/// ```json
/// {"success": false, "error": "error message"}
/// ```
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * `toml_str` must be a valid null-terminated UTF-8 string
#[no_mangle]
pub extern "C" fn praeda_generator_load_toml(
    handle: *mut PraedaGeneratorHandle,
    toml_str: *const c_char,
) -> *mut c_char {
    let result: std::result::Result<serde_json::Value, String> = (|| {
        if handle.is_null() {
            return Err("Invalid handle".to_string());
        }

        let toml_cstr = unsafe { CStr::from_ptr(toml_str) };
        let toml_string = toml_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in TOML string".to_string())?;

        let gen = unsafe { &mut (*handle).generator };
        gen.load_data_toml(toml_string)
            .map_err(|e| format!("Failed to load TOML: {}", e))?;

        Ok(serde_json::json!({ "success": true }))
    })();

    match result {
        Ok(json) => {
            let json_str = json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
        Err(e) => {
            let error_json = serde_json::json!({ "success": false, "error": e });
            let json_str = error_json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

// ============================================================================
// Programmatic Configuration (Alternative to TOML)
// ============================================================================

/// Set quality tier data
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `quality_name` - Quality name as null-terminated C string (e.g., "common", "rare")
/// * `weight` - Weight/probability for this quality (higher = more likely)
///
/// # Returns
/// JSON object with status
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * `quality_name` must be a valid null-terminated UTF-8 string
#[no_mangle]
pub extern "C" fn praeda_generator_set_quality_data(
    handle: *mut PraedaGeneratorHandle,
    quality_name: *const c_char,
    weight: i32,
) -> *mut c_char {
    let result: std::result::Result<serde_json::Value, String> = (|| {
        if handle.is_null() {
            return Err("Invalid handle".to_string());
        }

        let quality_cstr = unsafe { CStr::from_ptr(quality_name) };
        let quality_str = quality_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in quality name".to_string())?;

        let gen = unsafe { &mut (*handle).generator };
        gen.set_quality_data(quality_str.to_string(), weight);

        Ok(serde_json::json!({ "success": true }))
    })();

    match result {
        Ok(json) => {
            let json_str = json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
        Err(e) => {
            let error_json = serde_json::json!({ "success": false, "error": e });
            let json_str = error_json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

/// Set item type with weight
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `type_name` - Item type name as null-terminated C string (e.g., "weapon", "armor")
/// * `weight` - Weight/probability for this item type
///
/// # Returns
/// JSON object with status
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * `type_name` must be a valid null-terminated UTF-8 string
#[no_mangle]
pub extern "C" fn praeda_generator_set_item_type(
    handle: *mut PraedaGeneratorHandle,
    type_name: *const c_char,
    weight: i32,
) -> *mut c_char {
    let result: std::result::Result<serde_json::Value, String> = (|| {
        if handle.is_null() {
            return Err("Invalid handle".to_string());
        }

        let type_cstr = unsafe { CStr::from_ptr(type_name) };
        let type_str = type_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in type name".to_string())?;

        let gen = unsafe { &mut (*handle).generator };
        gen.set_item_type(type_str.to_string(), weight);

        Ok(serde_json::json!({ "success": true }))
    })();

    match result {
        Ok(json) => {
            let json_str = json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
        Err(e) => {
            let error_json = serde_json::json!({ "success": false, "error": e });
            let json_str = error_json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

/// Set item subtype with weight
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `type_name` - Item type name as null-terminated C string
/// * `subtype_name` - Item subtype name as null-terminated C string (e.g., "sword", "axe")
/// * `weight` - Weight/probability for this subtype
///
/// # Returns
/// JSON object with status
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * `type_name` and `subtype_name` must be valid null-terminated UTF-8 strings
#[no_mangle]
pub extern "C" fn praeda_generator_set_item_subtype(
    handle: *mut PraedaGeneratorHandle,
    type_name: *const c_char,
    subtype_name: *const c_char,
    weight: i32,
) -> *mut c_char {
    let result: std::result::Result<serde_json::Value, String> = (|| {
        if handle.is_null() {
            return Err("Invalid handle".to_string());
        }

        let type_cstr = unsafe { CStr::from_ptr(type_name) };
        let type_str = type_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in type name".to_string())?;

        let subtype_cstr = unsafe { CStr::from_ptr(subtype_name) };
        let subtype_str = subtype_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in subtype name".to_string())?;

        let gen = unsafe { &mut (*handle).generator };
        gen.set_item_subtype(type_str.to_string(), subtype_str.to_string(), weight);

        Ok(serde_json::json!({ "success": true }))
    })();

    match result {
        Ok(json) => {
            let json_str = json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
        Err(e) => {
            let error_json = serde_json::json!({ "success": false, "error": e });
            let json_str = error_json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

/// Set attribute for an item type/subtype
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `type_name` - Item type name as null-terminated C string
/// * `subtype_name` - Item subtype name (use empty string "" for type-wide attributes)
/// * `attr_name` - Attribute name as null-terminated C string (e.g., "damage", "defense")
/// * `initial_value` - Starting value for the attribute
/// * `min_value` - Minimum value (clamping floor)
/// * `max_value` - Maximum value (clamping ceiling)
/// * `required` - Whether this attribute is required (1 = true, 0 = false)
///
/// # Returns
/// JSON object with status
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * `type_name`, `subtype_name`, and `attr_name` must be valid null-terminated UTF-8 strings
#[no_mangle]
pub extern "C" fn praeda_generator_set_attribute(
    handle: *mut PraedaGeneratorHandle,
    type_name: *const c_char,
    subtype_name: *const c_char,
    attr_name: *const c_char,
    initial_value: f64,
    min_value: f64,
    max_value: f64,
    required: i32,
) -> *mut c_char {
    let result: std::result::Result<serde_json::Value, String> = (|| {
        if handle.is_null() {
            return Err("Invalid handle".to_string());
        }

        let type_cstr = unsafe { CStr::from_ptr(type_name) };
        let type_str = type_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in type name".to_string())?;

        let subtype_cstr = unsafe { CStr::from_ptr(subtype_name) };
        let subtype_str = subtype_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in subtype name".to_string())?;

        let attr_cstr = unsafe { CStr::from_ptr(attr_name) };
        let attr_str = attr_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in attribute name".to_string())?;

        let attr = ItemAttribute::new(
            attr_str.to_string(),
            initial_value,
            min_value,
            max_value,
            required != 0,
        );

        let gen = unsafe { &mut (*handle).generator };
        gen.set_attribute(type_str.to_string(), subtype_str.to_string(), attr);

        Ok(serde_json::json!({ "success": true }))
    })();

    match result {
        Ok(json) => {
            let json_str = json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
        Err(e) => {
            let error_json = serde_json::json!({ "success": false, "error": e });
            let json_str = error_json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

/// Set item names for a type/subtype combination
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `type_name` - Item type name as null-terminated C string
/// * `subtype_name` - Item subtype name as null-terminated C string
/// * `names_json` - JSON array of item names as null-terminated C string
///   Example: `"[\"longsword\", \"shortsword\"]"`
///
/// # Returns
/// JSON object with status
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * All string pointers must be valid null-terminated UTF-8 strings
/// * `names_json` must be a valid JSON array of strings
#[no_mangle]
pub extern "C" fn praeda_generator_set_item_names(
    handle: *mut PraedaGeneratorHandle,
    type_name: *const c_char,
    subtype_name: *const c_char,
    names_json: *const c_char,
) -> *mut c_char {
    let result: std::result::Result<serde_json::Value, String> = (|| {
        if handle.is_null() {
            return Err("Invalid handle".to_string());
        }

        let type_cstr = unsafe { CStr::from_ptr(type_name) };
        let type_str = type_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in type name".to_string())?;

        let subtype_cstr = unsafe { CStr::from_ptr(subtype_name) };
        let subtype_str = subtype_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in subtype name".to_string())?;

        let names_cstr = unsafe { CStr::from_ptr(names_json) };
        let names_string = names_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in names JSON".to_string())?;

        let names: Vec<String> = serde_json::from_str(names_string)
            .map_err(|e| format!("Failed to parse names JSON: {}", e))?;

        let gen = unsafe { &mut (*handle).generator };
        gen.set_item(type_str.to_string(), subtype_str.to_string(), names);

        Ok(serde_json::json!({ "success": true }))
    })();

    match result {
        Ok(json) => {
            let json_str = json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
        Err(e) => {
            let error_json = serde_json::json!({ "success": false, "error": e });
            let json_str = error_json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

// ============================================================================
// Loot Generation
// ============================================================================

/// Generate loot items
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `options_json` - Generation options as JSON string
///
/// # Returns
/// JSON array of generated items, or error object
///
/// # Options JSON Format
/// ```json
/// {
///   "number_of_items": 10,
///   "base_level": 15.0,
///   "level_variance": 5.0,
///   "affix_chance": 0.75,
///   "linear": true,
///   "scaling_factor": 1.0
/// }
/// ```
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * `options_json` must be a valid null-terminated UTF-8 string
#[no_mangle]
pub extern "C" fn praeda_generator_generate_loot(
    handle: *mut PraedaGeneratorHandle,
    options_json: *const c_char,
) -> *mut c_char {
    let result: std::result::Result<serde_json::Value, String> = (|| {
        if handle.is_null() {
            return Err("Invalid handle".to_string());
        }

        let options_cstr = unsafe { CStr::from_ptr(options_json) };
        let options_string = options_cstr
            .to_str()
            .map_err(|_| "Invalid UTF-8 in options JSON".to_string())?;

        let options: GeneratorOptions = serde_json::from_str(options_string)
            .map_err(|e| format!("Failed to parse options: {}", e))?;

        let gen = unsafe { &mut (*handle).generator };
        let items = gen
            .generate_loot(&options, &GeneratorOverrides::empty(), "ffi")
            .map_err(|e| e.to_string())?;

        Ok(serde_json::to_value(items).unwrap_or(serde_json::Value::Array(vec![])))
    })();

    match result {
        Ok(json) => {
            let json_str = json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
        Err(e) => {
            let error_json = serde_json::json!({
                "error": e,
                "success": false
            });
            let json_str = error_json.to_string();
            CString::new(json_str)
                .ok()
                .map(|s| s.into_raw() as *mut c_char)
                .unwrap_or(std::ptr::null_mut())
        }
    }
}

// ============================================================================
// String Management
// ============================================================================

/// Free a string returned by the FFI
///
/// All string pointers returned by praeda_* functions should be freed with this function.
///
/// # Arguments
/// * `ptr` - String pointer from FFI function
///
/// # Safety
/// The pointer must have been returned by a praeda_* function and not previously freed.
#[no_mangle]
pub extern "C" fn praeda_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

// ============================================================================
// Query Methods
// ============================================================================

/// Get all quality data as JSON
///
/// # Arguments
/// * `handle` - Pointer to generator
///
/// # Returns
/// JSON object mapping quality names to weights
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
#[no_mangle]
pub extern "C" fn praeda_generator_get_quality_data(
    handle: *mut PraedaGeneratorHandle,
) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let gen = unsafe { &(*handle).generator };
    let quality_data = gen.get_quality_data();
    let json = serde_json::to_value(quality_data).unwrap_or(serde_json::Value::Object(
        serde_json::Map::new(),
    ));

    let json_str = json.to_string();
    CString::new(json_str)
        .ok()
        .map(|s| s.into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

/// Check if a quality exists
///
/// # Arguments
/// * `handle` - Pointer to generator
/// * `quality` - Quality name as null-terminated C string
///
/// # Returns
/// 1 if quality exists, 0 if not, -1 on error
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
/// * `quality` must be a valid null-terminated UTF-8 string
#[no_mangle]
pub extern "C" fn praeda_generator_has_quality(
    handle: *mut PraedaGeneratorHandle,
    quality: *const c_char,
) -> i32 {
    if handle.is_null() {
        return -1;
    }

    match unsafe { CStr::from_ptr(quality).to_str() } {
        Ok(quality_str) => {
            let gen = unsafe { &(*handle).generator };
            if gen.has_quality(quality_str) {
                1
            } else {
                0
            }
        }
        Err(_) => -1,
    }
}

// ============================================================================
// Version and Info
// ============================================================================

/// Get Praeda library version
///
/// # Returns
/// Version string as null-terminated C string (e.g., "0.1.5")
///
/// # Safety
/// The returned string must be freed with praeda_string_free()
#[no_mangle]
pub extern "C" fn praeda_version() -> *mut c_char {
    let version = env!("CARGO_PKG_VERSION");
    CString::new(version)
        .ok()
        .map(|s| s.into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}

/// Get a human-readable string describing the generator state
///
/// # Arguments
/// * `handle` - Pointer to generator
///
/// # Returns
/// JSON object with generator info
///
/// # Safety
/// * `handle` must be a valid pointer from praeda_generator_new()
#[no_mangle]
pub extern "C" fn praeda_generator_info(handle: *mut PraedaGeneratorHandle) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let gen = unsafe { &(*handle).generator };
    let info = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "qualities": gen.get_quality_data().len(),
        "item_types": gen.get_item_types().len(),
    });

    let json_str = info.to_string();
    CString::new(json_str)
        .ok()
        .map(|s| s.into_raw() as *mut c_char)
        .unwrap_or(std::ptr::null_mut())
}
