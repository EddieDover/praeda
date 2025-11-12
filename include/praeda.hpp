/// Praeda C++ Wrapper
///
/// High-level C++ interface to the Praeda Rust loot generation library
/// Uses nlohmann/json for JSON handling

#pragma once

#include <string>
#include <memory>
#include <stdexcept>
#include <cstdint>

// Forward declare C FFI functions
extern "C" {
    // Memory management
    void* praeda_generator_new();
    void praeda_generator_free(void* handle);
    void praeda_string_free(char* ptr);

    // Configuration
    char* praeda_generator_load_toml(void* handle, const char* toml_str);

    // Generation
    char* praeda_generator_generate_loot(void* handle, const char* options_json);

    // Queries
    char* praeda_generator_get_quality_data(void* handle);
    int praeda_generator_has_quality(void* handle, const char* quality);

    // Info
    char* praeda_version();
    char* praeda_generator_info(void* handle);
}

// Praeda namespace
namespace praeda {

/// RAII wrapper for C strings from FFI
class CString {
public:
    explicit CString(char* ptr) : ptr_(ptr) {}
    ~CString() {
        if (ptr_) {
            praeda_string_free(ptr_);
        }
    }

    // Delete copy operations
    CString(const CString&) = delete;
    CString& operator=(const CString&) = delete;

    // Allow move operations
    CString(CString&& other) noexcept : ptr_(other.release()) {}
    CString& operator=(CString&& other) noexcept {
        if (ptr_) praeda_string_free(ptr_);
        ptr_ = other.release();
        return *this;
    }

    const char* c_str() const { return ptr_; }
    std::string str() const { return std::string(ptr_ ? ptr_ : ""); }

    char* release() {
        char* temp = ptr_;
        ptr_ = nullptr;
        return temp;
    }

private:
    char* ptr_;
};

/// Exception thrown by Praeda FFI
class Exception : public std::runtime_error {
public:
    explicit Exception(const std::string& what) : std::runtime_error(what) {}
};

/// Main Praeda generator class
///
/// Usage example:
/// ```cpp
/// #include "praeda.hpp"
/// #include <nlohmann/json.hpp>
///
/// using json = nlohmann::json;
///
/// int main() {
///     try {
///         auto gen = praeda::Generator::create();
///
///         // Load configuration
///         gen->load_toml_string(R"(
///             [quality_data]
///             common = 100
///             uncommon = 60
///             rare = 30
///         )");
///
///         // Generate items
///         json options = {
///             {"number_of_items", 10},
///             {"base_level", 15.0},
///             {"affix_chance", 0.75},
///             {"linear", true},
///             {"scaling_factor", 1.0}
///         };
///
///         json items = gen->generate_loot(options);
///         std::cout << items.dump(2) << std::endl;
///
///     } catch (const praeda::Exception& e) {
///         std::cerr << "Error: " << e.what() << std::endl;
///         return 1;
///     }
///     return 0;
/// }
/// ```
class Generator {
public:
    /// Create a new Praeda generator
    static std::unique_ptr<Generator> create() {
        void* handle = praeda_generator_new();
        if (!handle) {
            throw Exception("Failed to create generator");
        }
        return std::unique_ptr<Generator>(new Generator(handle));
    }

    /// Destructor
    ~Generator() {
        if (handle_) {
            praeda_generator_free(handle_);
        }
    }

    // Delete copy operations
    Generator(const Generator&) = delete;
    Generator& operator=(const Generator&) = delete;

    // Allow move operations
    Generator(Generator&& other) noexcept : handle_(other.release()) {}
    Generator& operator=(Generator&& other) noexcept {
        if (handle_) praeda_generator_free(handle_);
        handle_ = other.release();
        return *this;
    }

    /// Load configuration from TOML string
    ///
    /// Example TOML:
    /// ```toml
    /// [quality_data]
    /// common = 100
    /// uncommon = 60
    /// rare = 30
    ///
    /// [[item_types]]
    /// item_type = "weapon"
    /// weight = 1
    /// [item_types.subtypes]
    /// sword = 1
    /// ```
    void load_toml_string(const std::string& toml_content) {
        CString result(praeda_generator_load_toml(handle_, toml_content.c_str()));
        check_result(result.str());
    }

    /// Generate loot items
    ///
    /// Returns JSON array of generated items.
    /// Each item includes: name, quality, type, subtype, attributes, prefix, suffix
    std::string generate_loot(const std::string& options_json) {
        CString result(praeda_generator_generate_loot(handle_, options_json.c_str()));
        if (!result.c_str()) {
            throw Exception("Failed to generate loot");
        }
        return result.str();
    }

    /// Get quality data
    std::string get_quality_data() {
        CString result(praeda_generator_get_quality_data(handle_));
        return result.str();
    }

    /// Check if a quality exists
    bool has_quality(const std::string& quality) {
        int result = praeda_generator_has_quality(handle_, quality.c_str());
        if (result < 0) {
            throw Exception("Error checking quality");
        }
        return result == 1;
    }

    /// Get generator info
    std::string info() {
        CString result(praeda_generator_info(handle_));
        return result.str();
    }

    /// Get the underlying FFI handle (advanced usage only)
    void* native_handle() const { return handle_; }

private:
    void* handle_;

    explicit Generator(void* handle) : handle_(handle) {}

    void* release() {
        void* temp = handle_;
        handle_ = nullptr;
        return temp;
    }

    void check_result(const std::string& json_response) {
        // Very basic JSON parsing without external dependency
        if (json_response.find("\"success\":false") != std::string::npos ||
            json_response.find("\"error\"") != std::string::npos) {
            // Extract error message if present
            size_t error_pos = json_response.find("\"error\":\"");
            if (error_pos != std::string::npos) {
                size_t start = error_pos + 9;
                size_t end = json_response.find("\"", start);
                if (end != std::string::npos) {
                    std::string error_msg = json_response.substr(start, end - start);
                    throw Exception(error_msg);
                }
            }
            throw Exception("Operation failed");
        }
    }
};

/// Get Praeda library version
inline std::string version() {
    CString v(praeda_version());
    return v.str();
}

} // namespace praeda
