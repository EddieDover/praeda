//! Foreign Function Interface (FFI) for Praeda
//!
//! This module provides C-compatible bindings for the Praeda loot generation library.
//! It allows usage from C++, C#, and other languages that can call C functions.
//!
//! All data is exchanged through:
//! - Simple C types (int, char*, pointers)
//! - C-compatible structs and arrays
//! - Opaque handles for complex types

#![allow(clippy::not_unsafe_ptr_arg_deref)]

use crate::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint};

// ============================================================================
// C-Compatible Struct Definitions
// ============================================================================

/// C-compatible representation of ItemAttribute
#[repr(C)]
pub struct CItemAttribute {
    pub name: *mut c_char,
    pub initial_value: f64,
    pub min: f64,
    pub max: f64,
    pub required: u8,
    pub scaling_factor: f64,
    pub chance: f64,
}

impl CItemAttribute {
    fn from_rust(attr: &ItemAttribute) -> Self {
        CItemAttribute {
            name: CString::new(attr.name.clone())
                .ok()
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut()),
            initial_value: attr.initial_value,
            min: attr.min,
            max: attr.max,
            required: if attr.required { 1 } else { 0 },
            scaling_factor: attr.scaling_factor,
            chance: attr.chance,
        }
    }

    fn free(&mut self) {
        if !self.name.is_null() {
            unsafe {
                let _ = CString::from_raw(self.name);
            }
            self.name = std::ptr::null_mut();
        }
    }
}

/// C-compatible representation of Affix
#[repr(C)]
pub struct CAffix {
    pub name: *mut c_char,
    pub attributes: *mut CItemAttribute,
    pub attributes_count: c_uint,
}

impl CAffix {
    fn from_rust(affix: &Affix) -> Self {
        let attributes: Vec<CItemAttribute> = affix.attributes.iter().map(CItemAttribute::from_rust).collect();
        let count = attributes.len() as c_uint;
        let attrs_ptr = if count > 0 {
            Box::into_raw(attributes.into_boxed_slice()) as *mut CItemAttribute
        } else {
            std::ptr::null_mut()
        };

        CAffix {
            name: CString::new(affix.name.clone())
                .ok()
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut()),
            attributes: attrs_ptr,
            attributes_count: count,
        }
    }

    fn free(&mut self) {
        if !self.name.is_null() {
            unsafe {
                let _ = CString::from_raw(self.name);
            }
            self.name = std::ptr::null_mut();
        }

        if !self.attributes.is_null() && self.attributes_count > 0 {
            unsafe {
                for i in 0..self.attributes_count {
                    (*self.attributes.add(i as usize)).free();
                }
                let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.attributes,
                    self.attributes_count as usize,
                ));
            }
            self.attributes = std::ptr::null_mut();
            self.attributes_count = 0;
        }
    }
}

/// C-compatible representation of Item
#[repr(C)]
pub struct CItem {
    pub name: *mut c_char,
    pub quality: *mut c_char,
    pub item_type: *mut c_char,
    pub subtype: *mut c_char,
    pub prefix: CAffix,
    pub suffix: CAffix,
    pub attributes: *mut CItemAttribute,
    pub attributes_count: c_uint,
}

impl CItem {
    fn from_rust(item: &Item) -> Self {
        let attributes: Vec<CItemAttribute> = item
            .attributes
            .values()
            .map(CItemAttribute::from_rust)
            .collect();
        let attr_count = attributes.len() as c_uint;
        let attrs_ptr = if attr_count > 0 {
            Box::into_raw(attributes.into_boxed_slice()) as *mut CItemAttribute
        } else {
            std::ptr::null_mut()
        };

        CItem {
            name: CString::new(item.name.clone())
                .ok()
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut()),
            quality: CString::new(item.quality.clone())
                .ok()
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut()),
            item_type: CString::new(item.item_type.clone())
                .ok()
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut()),
            subtype: CString::new(item.subtype.clone())
                .ok()
                .map(|s| s.into_raw())
                .unwrap_or(std::ptr::null_mut()),
            prefix: CAffix::from_rust(&item.prefix),
            suffix: CAffix::from_rust(&item.suffix),
            attributes: attrs_ptr,
            attributes_count: attr_count,
        }
    }

    fn free(&mut self) {
        if !self.name.is_null() {
            unsafe {
                let _ = CString::from_raw(self.name);
            }
            self.name = std::ptr::null_mut();
        }

        if !self.quality.is_null() {
            unsafe {
                let _ = CString::from_raw(self.quality);
            }
            self.quality = std::ptr::null_mut();
        }

        if !self.item_type.is_null() {
            unsafe {
                let _ = CString::from_raw(self.item_type);
            }
            self.item_type = std::ptr::null_mut();
        }

        if !self.subtype.is_null() {
            unsafe {
                let _ = CString::from_raw(self.subtype);
            }
            self.subtype = std::ptr::null_mut();
        }

        self.prefix.free();
        self.suffix.free();

        if !self.attributes.is_null() && self.attributes_count > 0 {
            unsafe {
                for i in 0..self.attributes_count {
                    (*self.attributes.add(i as usize)).free();
                }
                let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.attributes,
                    self.attributes_count as usize,
                ));
            }
            self.attributes = std::ptr::null_mut();
            self.attributes_count = 0;
        }
    }
}

/// C-compatible array of Items
#[repr(C)]
pub struct CItemArray {
    pub items: *mut CItem,
    pub count: c_uint,
}

impl CItemArray {
    fn from_rust(items: &[Item]) -> Self {
        let c_items: Vec<CItem> = items.iter().map(CItem::from_rust).collect();
        let count = c_items.len() as c_uint;
        let items_ptr = if count > 0 {
            Box::into_raw(c_items.into_boxed_slice()) as *mut CItem
        } else {
            std::ptr::null_mut()
        };

        CItemArray {
            items: items_ptr,
            count,
        }
    }
}

/// Opaque pointer to a PraedaGenerator instance
pub struct PraedaGeneratorHandle {
    generator: Box<PraedaGenerator>,
}

/// Opaque pointer to an array of Items
pub struct CItemArrayHandle {
    array: CItemArray,
}

// ============================================================================
// Memory Management
// ============================================================================

/// Create a new Praeda generator
#[no_mangle]
pub extern "C" fn praeda_generator_new() -> *mut PraedaGeneratorHandle {
    Box::into_raw(Box::new(PraedaGeneratorHandle {
        generator: Box::new(PraedaGenerator::new()),
    }))
}

/// Free a Praeda generator instance
#[no_mangle]
pub extern "C" fn praeda_generator_free(handle: *mut PraedaGeneratorHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        }
    }
}

/// Free a C string allocated by FFI
#[no_mangle]
pub extern "C" fn praeda_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// Free an error string
#[no_mangle]
pub extern "C" fn praeda_error_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// Free an item array handle
#[no_mangle]
pub extern "C" fn praeda_item_array_free(handle: *mut CItemArrayHandle) {
    if !handle.is_null() {
        unsafe {
            let array_handle = Box::from_raw(handle);
            if !array_handle.array.items.is_null() && array_handle.array.count > 0 {
                for i in 0..array_handle.array.count {
                    (*array_handle.array.items.add(i as usize)).free();
                }
                let _ = Box::from_raw(std::slice::from_raw_parts_mut(
                    array_handle.array.items,
                    array_handle.array.count as usize,
                ));
            }
        }
    }
}

// ============================================================================
// TOML Configuration
// ============================================================================

/// Load configuration from a TOML string
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn praeda_generator_load_toml(
    handle: *mut PraedaGeneratorHandle,
    toml_str: *const c_char,
    error_out: *mut *mut c_char,
) -> i32 {
    if handle.is_null() || toml_str.is_null() {
        if !error_out.is_null() {
            if let Ok(err) = CString::new("Invalid handle or TOML string") {
                unsafe {
                    *error_out = err.into_raw();
                }
            }
        }
        return -1;
    }

    let toml_cstr = unsafe { CStr::from_ptr(toml_str) };
    let toml_string = match toml_cstr.to_str() {
        Ok(s) => s,
        Err(_) => {
            if !error_out.is_null() {
                if let Ok(err) = CString::new("Invalid UTF-8 in TOML string") {
                    unsafe {
                        *error_out = err.into_raw();
                    }
                }
            }
            return -1;
        }
    };

    let gen = unsafe { &mut (*handle).generator };
    match gen.load_data_toml(toml_string) {
        Ok(_) => 0,
        Err(e) => {
            if !error_out.is_null() {
                if let Ok(err) = CString::new(format!("Failed to load TOML: {}", e)) {
                    unsafe {
                        *error_out = err.into_raw();
                    }
                }
            }
            -1
        }
    }
}

// ============================================================================
// Programmatic Configuration
// ============================================================================

/// Set quality tier data
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn praeda_generator_set_quality_data(
    handle: *mut PraedaGeneratorHandle,
    quality_name: *const c_char,
    weight: i32,
) -> i32 {
    if handle.is_null() || quality_name.is_null() {
        return -1;
    }

    let quality_cstr = unsafe { CStr::from_ptr(quality_name) };
    let quality_str = match quality_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let gen = unsafe { &mut (*handle).generator };
    gen.set_quality_data(quality_str.to_string(), weight);
    0
}

/// Set item type with weight
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn praeda_generator_set_item_type(
    handle: *mut PraedaGeneratorHandle,
    type_name: *const c_char,
    weight: i32,
) -> i32 {
    if handle.is_null() || type_name.is_null() {
        return -1;
    }

    let type_cstr = unsafe { CStr::from_ptr(type_name) };
    let type_str = match type_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let gen = unsafe { &mut (*handle).generator };
    gen.set_item_type(type_str.to_string(), weight);
    0
}

/// Set item subtype with weight
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn praeda_generator_set_item_subtype(
    handle: *mut PraedaGeneratorHandle,
    type_name: *const c_char,
    subtype_name: *const c_char,
    weight: i32,
) -> i32 {
    if handle.is_null() || type_name.is_null() || subtype_name.is_null() {
        return -1;
    }

    let type_cstr = unsafe { CStr::from_ptr(type_name) };
    let type_str = match type_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let subtype_cstr = unsafe { CStr::from_ptr(subtype_name) };
    let subtype_str = match subtype_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let gen = unsafe { &mut (*handle).generator };
    gen.set_item_subtype(type_str.to_string(), subtype_str.to_string(), weight);
    0
}

/// Set attribute for an item type/subtype
/// Returns 0 on success, -1 on failure
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
) -> i32 {
    if handle.is_null() || type_name.is_null() || subtype_name.is_null() || attr_name.is_null() {
        return -1;
    }

    let type_cstr = unsafe { CStr::from_ptr(type_name) };
    let type_str = match type_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let subtype_cstr = unsafe { CStr::from_ptr(subtype_name) };
    let subtype_str = match subtype_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let attr_cstr = unsafe { CStr::from_ptr(attr_name) };
    let attr_str = match attr_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let attribute = ItemAttribute::new(
        attr_str.to_string(),
        initial_value,
        min_value,
        max_value,
        required != 0,
    );

    let gen = unsafe { &mut (*handle).generator };
    gen.set_attribute(
        type_str.to_string(),
        subtype_str.to_string(),
        attribute,
    );
    0
}

/// Set item names for a type/subtype combination
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn praeda_generator_set_item_names(
    handle: *mut PraedaGeneratorHandle,
    type_name: *const c_char,
    subtype_name: *const c_char,
    names: *const *const c_char,
    names_count: c_uint,
) -> i32 {
    if handle.is_null() || type_name.is_null() || subtype_name.is_null() || names.is_null() {
        return -1;
    }

    let type_cstr = unsafe { CStr::from_ptr(type_name) };
    let type_str = match type_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let subtype_cstr = unsafe { CStr::from_ptr(subtype_name) };
    let subtype_str = match subtype_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let mut names_vec = Vec::new();
    for i in 0..names_count as usize {
        let name_ptr = unsafe { *names.add(i) };
        if name_ptr.is_null() {
            return -1;
        }
        let name_cstr = unsafe { CStr::from_ptr(name_ptr) };
        match name_cstr.to_str() {
            Ok(s) => names_vec.push(s.to_string()),
            Err(_) => return -1,
        }
    }

    let gen = unsafe { &mut (*handle).generator };
    gen.set_item(type_str.to_string(), subtype_str.to_string(), names_vec);
    0
}

// ============================================================================
// Loot Generation
// ============================================================================

/// Generate loot items with options
/// Returns handle to CItemArray on success, null on failure
#[no_mangle]
pub extern "C" fn praeda_generator_generate_loot(
    handle: *mut PraedaGeneratorHandle,
    number_of_items: c_uint,
    base_level: f64,
    level_variance: f64,
    affix_chance: f64,
    linear: u8,
    scaling_factor: f64,
    error_out: *mut *mut c_char,
) -> *mut CItemArrayHandle {
    if handle.is_null() {
        if !error_out.is_null() {
            if let Ok(err) = CString::new("Invalid handle") {
                unsafe {
                    *error_out = err.into_raw();
                }
            }
        }
        return std::ptr::null_mut();
    }

    let options = GeneratorOptions {
        number_of_items,
        base_level,
        level_variance,
        affix_chance,
        linear: linear != 0,
        scaling_factor,
    };

    let gen = unsafe { &mut (*handle).generator };
    match gen.generate_loot(&options, &GeneratorOverrides::empty(), "ffi") {
        Ok(items) => {
            let c_array = CItemArray::from_rust(&items);
            Box::into_raw(Box::new(CItemArrayHandle { array: c_array }))
        }
        Err(e) => {
            if !error_out.is_null() {
                if let Ok(err) = CString::new(format!("Failed to generate loot: {}", e)) {
                    unsafe {
                        *error_out = err.into_raw();
                    }
                }
            }
            std::ptr::null_mut()
        }
    }
}

/// Get items from array handle
/// Panics if handle is invalid - caller must ensure handle is valid
#[no_mangle]
pub extern "C" fn praeda_item_array_get(
    handle: *const CItemArrayHandle,
    index: c_uint,
) -> *const CItem {
    if handle.is_null() {
        return std::ptr::null();
    }

    let array_handle = unsafe { &*handle };
    if index >= array_handle.array.count || array_handle.array.items.is_null() {
        return std::ptr::null();
    }

    unsafe { &*array_handle.array.items.add(index as usize) }
}

/// Get item array count
#[no_mangle]
pub extern "C" fn praeda_item_array_count(handle: *const CItemArrayHandle) -> c_uint {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).array.count }
}

// ============================================================================
// Query Methods
// ============================================================================

/// Check if a quality exists
/// Returns 1 if exists, 0 if not, -1 on error
#[no_mangle]
pub extern "C" fn praeda_generator_has_quality(
    handle: *const PraedaGeneratorHandle,
    quality: *const c_char,
) -> i32 {
    if handle.is_null() || quality.is_null() {
        return -1;
    }

    let quality_cstr = unsafe { CStr::from_ptr(quality) };
    let quality_str = match quality_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let gen = unsafe { &(*handle).generator };
    if gen.has_quality(quality_str) {
        1
    } else {
        0
    }
}

/// Get version string
/// Caller must free returned string with praeda_string_free()
#[no_mangle]
pub extern "C" fn praeda_version() -> *mut c_char {
    if let Ok(version) = CString::new(env!("CARGO_PKG_VERSION")) {
        version.into_raw()
    } else {
        std::ptr::null_mut()
    }
}
