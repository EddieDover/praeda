//! Unit tests for the struct-based FFI API
//!
//! These tests verify the C-compatible FFI interface with struct-based data exchange.
//! All tests use the new API that returns integers (0 = success, -1 = failure) and
//! C-compatible structs instead of JSON strings.

#![allow(unsafe_code, clippy::useless_vec, unused_unsafe)]

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

// ============================================================================
// Basic Generator Lifecycle Tests
// ============================================================================

#[test]
fn test_generator_create_and_free() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null(), "Generator handle should not be null");

        praeda_generator_free(handle);
        // If we get here without crashing, the test passes
    }
}

#[test]
fn test_generator_null_handle_free() {
    unsafe {
        // Should not crash when freeing null handle
        praeda_generator_free(std::ptr::null_mut());
    }
}

#[test]
fn test_version() {
    unsafe {
        let version_ptr = praeda_version();
        assert!(!version_ptr.is_null(), "Version pointer should not be null");

        let version = c_str_to_string(version_ptr);
        assert!(!version.is_empty(), "Version string should not be empty");
        assert!(
            version.contains('.'),
            "Version should be in format X.Y.Z"
        );

        praeda_string_free(version_ptr as *mut c_char);
    }
}

// ============================================================================
// Configuration Tests
// ============================================================================

#[test]
fn test_set_quality_data() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Should succeed with valid quality
        let result =
            praeda_generator_set_quality_data(handle, CString::new("common").unwrap().as_ptr(), 100);
        assert_eq!(result, 0, "Setting quality should return 0 (success)");

        let result =
            praeda_generator_set_quality_data(handle, CString::new("rare").unwrap().as_ptr(), 30);
        assert_eq!(result, 0, "Setting another quality should succeed");

        praeda_generator_free(handle);
    }
}

#[test]
fn test_set_quality_data_null_handle() {
    unsafe {
        let invalid_handle: *mut PraedaGeneratorHandle = std::ptr::null_mut();

        let result =
            praeda_generator_set_quality_data(invalid_handle, CString::new("common").unwrap().as_ptr(), 100);
        assert_eq!(result, -1, "Setting quality on null handle should fail");
    }
}

#[test]
fn test_set_item_type() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        let result = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 2);
        assert_eq!(result, 0, "Setting item type should succeed");

        let result = praeda_generator_set_item_type(handle, CString::new("armor").unwrap().as_ptr(), 1);
        assert_eq!(result, 0, "Setting another item type should succeed");

        praeda_generator_free(handle);
    }
}

#[test]
fn test_set_item_subtype() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Set item type first
        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 1);

        // Then set subtype
        let result = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            3,
        );
        assert_eq!(result, 0, "Setting item subtype should succeed");

        praeda_generator_free(handle);
    }
}

#[test]
fn test_set_attribute() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Set item type first
        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 1);

        // Then set attribute
        let result = praeda_generator_set_attribute(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("").unwrap().as_ptr(),
            CString::new("damage").unwrap().as_ptr(),
            15.0,
            5.0,
            30.0,
            1, // required = true
        );
        assert_eq!(result, 0, "Setting attribute should succeed");

        praeda_generator_free(handle);
    }
}

#[test]
fn test_set_item_names() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Set type and subtype first
        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 1);
        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            1,
        );

        // Then set names
        let names = vec![
            CString::new("longsword").unwrap(),
            CString::new("shortsword").unwrap(),
        ];
        let name_ptrs: Vec<*const c_char> = names.iter().map(|s| s.as_ptr()).collect();

        let result = praeda_generator_set_item_names(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            name_ptrs.as_ptr(),
            name_ptrs.len() as u32,
        );
        assert_eq!(result, 0, "Setting item names should succeed");

        praeda_generator_free(handle);
    }
}

// ============================================================================
// Query Tests
// ============================================================================

#[test]
fn test_has_quality() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Set a quality
        let _ = praeda_generator_set_quality_data(handle, CString::new("common").unwrap().as_ptr(), 100);

        // Check that it exists
        let result = praeda_generator_has_quality(handle, CString::new("common").unwrap().as_ptr());
        assert_eq!(result, 1, "Should find quality that was set");

        // Check that non-existent quality returns 0
        let result = praeda_generator_has_quality(handle, CString::new("nonexistent").unwrap().as_ptr());
        assert_eq!(result, 0, "Should not find quality that wasn't set");

        praeda_generator_free(handle);
    }
}

#[test]
fn test_has_quality_null_handle() {
    unsafe {
        let invalid_handle: *mut PraedaGeneratorHandle = std::ptr::null_mut();

        let result = praeda_generator_has_quality(invalid_handle, CString::new("common").unwrap().as_ptr());
        assert_eq!(result, -1, "Checking quality on null handle should return error");
    }
}

// ============================================================================
// Loot Generation Tests with Struct-based API
// ============================================================================

#[test]
fn test_generate_loot_basic() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Configure generator
        let _ = praeda_generator_set_quality_data(handle, CString::new("common").unwrap().as_ptr(), 100);
        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 1);
        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            1,
        );

        let names = vec![CString::new("longsword").unwrap()];
        let name_ptrs: Vec<*const c_char> = names.iter().map(|s| s.as_ptr()).collect();
        let _ = praeda_generator_set_item_names(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            name_ptrs.as_ptr(),
            name_ptrs.len() as u32,
        );

        // Generate loot
        let mut error_ptr = std::ptr::null_mut();
        let array_handle = praeda_generator_generate_loot(
            handle,
            5,     // number_of_items
            10.0,  // base_level
            2.0,   // level_variance
            0.25,  // affix_chance
            1,     // linear
            1.0,   // scaling_factor
            &mut error_ptr,
        );

        assert!(
            !array_handle.is_null(),
            "Item array should not be null (error: {})",
            c_str_to_string(error_ptr)
        );

        // Check count
        let count = praeda_item_array_count(array_handle);
        assert_eq!(count, 5, "Should generate 5 items");

        // Check first item
        let item_ptr = praeda_item_array_get(array_handle, 0);
        assert!(!item_ptr.is_null(), "Item pointer should not be null");

        let item = unsafe { &*item_ptr };
        assert!(!item.name.is_null(), "Item name should not be null");
        assert!(!item.quality.is_null(), "Item quality should not be null");
        assert!(!item.item_type.is_null(), "Item type should not be null");
        assert!(!item.subtype.is_null(), "Item subtype should not be null");

        let name = c_str_to_string(item.name);
        let quality = c_str_to_string(item.quality);
        let item_type = c_str_to_string(item.item_type);
        let subtype = c_str_to_string(item.subtype);

        assert_eq!(name, "longsword", "Item name should match");
        assert_eq!(item_type, "weapon", "Item type should match");
        assert_eq!(subtype, "sword", "Item subtype should match");
        assert!(!quality.is_empty(), "Quality should not be empty");

        praeda_item_array_free(array_handle);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_generate_loot_multiple_items() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Configure generator with multiple options
        let _ = praeda_generator_set_quality_data(handle, CString::new("common").unwrap().as_ptr(), 100);
        let _ = praeda_generator_set_quality_data(handle, CString::new("rare").unwrap().as_ptr(), 20);

        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 1);
        let _ = praeda_generator_set_item_type(handle, CString::new("armor").unwrap().as_ptr(), 1);

        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            1,
        );
        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("armor").unwrap().as_ptr(),
            CString::new("chest").unwrap().as_ptr(),
            1,
        );

        // Set names for both
        let weapon_names = vec![CString::new("sword").unwrap()];
        let weapon_name_ptrs: Vec<*const c_char> = weapon_names.iter().map(|s| s.as_ptr()).collect();
        let _ = praeda_generator_set_item_names(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            weapon_name_ptrs.as_ptr(),
            weapon_name_ptrs.len() as u32,
        );

        let armor_names = vec![CString::new("platemail").unwrap()];
        let armor_name_ptrs: Vec<*const c_char> = armor_names.iter().map(|s| s.as_ptr()).collect();
        let _ = praeda_generator_set_item_names(
            handle,
            CString::new("armor").unwrap().as_ptr(),
            CString::new("chest").unwrap().as_ptr(),
            armor_name_ptrs.as_ptr(),
            armor_name_ptrs.len() as u32,
        );

        // Generate loot
        let mut error_ptr = std::ptr::null_mut();
        let array_handle = praeda_generator_generate_loot(
            handle,
            10,    // number_of_items
            15.0,  // base_level
            3.0,   // level_variance
            0.75,  // affix_chance
            1,     // linear
            1.0,   // scaling_factor
            &mut error_ptr,
        );

        assert!(!array_handle.is_null(), "Item array should not be null");

        let count = praeda_item_array_count(array_handle);
        assert_eq!(count, 10, "Should generate 10 items");

        // Verify all items have proper structure
        for i in 0..count {
            let item_ptr = praeda_item_array_get(array_handle, i);
            assert!(!item_ptr.is_null(), "Item {} should not be null", i);

            let item = unsafe { &*item_ptr };
            assert!(!item.name.is_null(), "Item {} name should not be null", i);
            assert!(!item.quality.is_null(), "Item {} quality should not be null", i);
            assert!(!item.item_type.is_null(), "Item {} type should not be null", i);
        }

        praeda_item_array_free(array_handle);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_generate_loot_null_handle() {
    unsafe {
        let invalid_handle: *mut PraedaGeneratorHandle = std::ptr::null_mut();
        let mut error_ptr = std::ptr::null_mut();

        let array_handle = praeda_generator_generate_loot(
            invalid_handle,
            5,     // number_of_items
            10.0,  // base_level
            2.0,   // level_variance
            0.25,  // affix_chance
            1,     // linear
            1.0,   // scaling_factor
            &mut error_ptr,
        );

        assert!(
            array_handle.is_null(),
            "Should return null for invalid handle"
        );
    }
}

#[test]
fn test_generate_loot_with_affix() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Configure with attributes for affixes
        let _ = praeda_generator_set_quality_data(handle, CString::new("common").unwrap().as_ptr(), 100);
        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 1);
        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            1,
        );

        // Set attribute for affix
        let _ = praeda_generator_set_attribute(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("").unwrap().as_ptr(),
            CString::new("damage").unwrap().as_ptr(),
            15.0,
            5.0,
            30.0,
            1,
        );

        let names = vec![CString::new("sword").unwrap()];
        let name_ptrs: Vec<*const c_char> = names.iter().map(|s| s.as_ptr()).collect();
        let _ = praeda_generator_set_item_names(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            name_ptrs.as_ptr(),
            name_ptrs.len() as u32,
        );

        // Generate with high affix chance
        let mut error_ptr = std::ptr::null_mut();
        let array_handle = praeda_generator_generate_loot(
            handle,
            10,    // number_of_items
            20.0,  // base_level
            5.0,   // level_variance
            1.0,   // affix_chance (100% for testing)
            1,     // linear
            1.0,   // scaling_factor
            &mut error_ptr,
        );

        assert!(!array_handle.is_null(), "Item array should not be null");

        let count = praeda_item_array_count(array_handle);
        assert_eq!(count, 10, "Should generate 10 items");

        praeda_item_array_free(array_handle);
        praeda_generator_free(handle);
    }
}

// ============================================================================
// TOML Configuration Tests
// ============================================================================

#[test]
fn test_load_toml_string() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        let toml_str = r#"
[quality_data]
common = 100
rare = 30

[[item_types]]
item_type = "weapon"
weight = 2
[item_types.subtypes]
sword = 3

[[item_list]]
item_type = "weapon"
subtype = "sword"
names = ["longsword"]
"#;

        let mut error_ptr = std::ptr::null_mut();
        let result = praeda_generator_load_toml(
            handle,
            CString::new(toml_str).unwrap().as_ptr(),
            &mut error_ptr,
        );

        assert_eq!(result, 0, "Loading valid TOML should succeed");
        assert!(
            error_ptr.is_null(),
            "Error pointer should be null on success"
        );

        // Verify qualities were loaded
        let has_common = praeda_generator_has_quality(handle, CString::new("common").unwrap().as_ptr());
        assert_eq!(has_common, 1, "Should have loaded 'common' quality");

        let has_rare = praeda_generator_has_quality(handle, CString::new("rare").unwrap().as_ptr());
        assert_eq!(has_rare, 1, "Should have loaded 'rare' quality");

        praeda_generator_free(handle);
    }
}

#[test]
fn test_load_toml_invalid_handle() {
    unsafe {
        let invalid_handle: *mut PraedaGeneratorHandle = std::ptr::null_mut();
        let toml_str = "[qualities]\ncommon = 100";

        let mut error_ptr = std::ptr::null_mut();
        let result = praeda_generator_load_toml(
            invalid_handle,
            CString::new(toml_str).unwrap().as_ptr(),
            &mut error_ptr,
        );

        assert_eq!(result, -1, "Loading TOML on null handle should fail");
    }
}

#[test]
fn test_load_toml_invalid_syntax() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        let invalid_toml = "this is not valid toml: {{[ }";

        let mut error_ptr = std::ptr::null_mut();
        let result = praeda_generator_load_toml(
            handle,
            CString::new(invalid_toml).unwrap().as_ptr(),
            &mut error_ptr,
        );

        assert_eq!(
            result, -1,
            "Loading invalid TOML should fail"
        );

        // Error message should be available
        if !error_ptr.is_null() {
            let error_msg = c_str_to_string(error_ptr);
            assert!(
                !error_msg.is_empty(),
                "Error message should be provided"
            );
            praeda_error_free(error_ptr as *mut c_char);
        }

        praeda_generator_free(handle);
    }
}

// ============================================================================
// Memory Management Tests
// ============================================================================

#[test]
fn test_string_free_null_pointer() {
    unsafe {
        // Should not crash when freeing null pointer
        praeda_string_free(std::ptr::null_mut());
    }
}

#[test]
fn test_error_free_null_pointer() {
    unsafe {
        // Should not crash when freeing null pointer
        praeda_error_free(std::ptr::null_mut());
    }
}

#[test]
fn test_item_array_free_null_pointer() {
    unsafe {
        // Should not crash when freeing null pointer
        praeda_item_array_free(std::ptr::null_mut());
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_full_workflow() {
    unsafe {
        // Create generator
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Configure via programmatic API
        let _ = praeda_generator_set_quality_data(handle, CString::new("common").unwrap().as_ptr(), 100);
        let _ = praeda_generator_set_quality_data(handle, CString::new("uncommon").unwrap().as_ptr(), 60);
        let _ = praeda_generator_set_quality_data(handle, CString::new("rare").unwrap().as_ptr(), 30);

        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 2);
        let _ = praeda_generator_set_item_type(handle, CString::new("armor").unwrap().as_ptr(), 1);

        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            3,
        );
        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("axe").unwrap().as_ptr(),
            2,
        );
        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("armor").unwrap().as_ptr(),
            CString::new("chest").unwrap().as_ptr(),
            1,
        );

        // Verify qualities exist
        assert_eq!(
            praeda_generator_has_quality(handle, CString::new("common").unwrap().as_ptr()),
            1
        );
        assert_eq!(
            praeda_generator_has_quality(handle, CString::new("rare").unwrap().as_ptr()),
            1
        );
        assert_eq!(
            praeda_generator_has_quality(handle, CString::new("epic").unwrap().as_ptr()),
            0
        );

        // Generate loot
        let mut error_ptr = std::ptr::null_mut();
        let array_handle = praeda_generator_generate_loot(
            handle,
            15,    // number_of_items
            20.0,  // base_level
            5.0,   // level_variance
            0.5,   // affix_chance
            1,     // linear
            1.0,   // scaling_factor
            &mut error_ptr,
        );

        assert!(!array_handle.is_null());
        assert_eq!(praeda_item_array_count(array_handle), 15);

        // Verify item structure
        for i in 0..praeda_item_array_count(array_handle) {
            let item_ptr = praeda_item_array_get(array_handle, i);
            let item = unsafe { &*item_ptr };

            let name = c_str_to_string(item.name);
            let quality = c_str_to_string(item.quality);
            let item_type = c_str_to_string(item.item_type);

            assert!(!name.is_empty());
            assert!(!quality.is_empty());
            assert!(!item_type.is_empty());
        }

        // Cleanup
        praeda_item_array_free(array_handle);
        praeda_generator_free(handle);
    }
}

#[test]
fn test_item_array_bounds() {
    unsafe {
        let handle = praeda_generator_new();
        assert!(!handle.is_null());

        // Configure minimal setup
        let _ = praeda_generator_set_quality_data(handle, CString::new("common").unwrap().as_ptr(), 100);
        let _ = praeda_generator_set_item_type(handle, CString::new("weapon").unwrap().as_ptr(), 1);
        let _ = praeda_generator_set_item_subtype(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            1,
        );

        let names = vec![CString::new("sword").unwrap()];
        let name_ptrs: Vec<*const c_char> = names.iter().map(|s| s.as_ptr()).collect();
        let _ = praeda_generator_set_item_names(
            handle,
            CString::new("weapon").unwrap().as_ptr(),
            CString::new("sword").unwrap().as_ptr(),
            name_ptrs.as_ptr(),
            name_ptrs.len() as u32,
        );

        // Generate 5 items
        let mut error_ptr = std::ptr::null_mut();
        let array_handle = praeda_generator_generate_loot(
            handle, 5, 10.0, 2.0, 0.25, 1, 1.0, &mut error_ptr,
        );

        let count = praeda_item_array_count(array_handle);
        assert_eq!(count, 5);

        // Access all items
        for i in 0..count {
            let item_ptr = praeda_item_array_get(array_handle, i);
            assert!(!item_ptr.is_null());
        }

        // Accessing beyond bounds should still work but might return invalid data
        // This is expected C behavior - callers are responsible for bounds checking
        let _out_of_bounds = praeda_item_array_get(array_handle, count + 10);
        // We don't assert about this - C FFI doesn't guarantee safety at boundaries

        praeda_item_array_free(array_handle);
        praeda_generator_free(handle);
    }
}
