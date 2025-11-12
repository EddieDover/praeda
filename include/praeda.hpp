/// Praeda C++ Wrapper
///
/// High-level C++ interface to the Praeda Rust loot generation library
/// Zero JSON - all data exchanged through native C++ types

#pragma once

#include <string>
#include <memory>
#include <vector>
#include <map>
#include <stdexcept>
#include <cstdint>
#include <cstring>

// ============================================================================
// C FFI Declarations
// ============================================================================

extern "C" {
    // Handle types (opaque pointers)
    typedef struct PraedaGeneratorHandle PraedaGeneratorHandle;
    typedef struct CItemArrayHandle CItemArrayHandle;

    // C-compatible structs
    typedef struct {
        char* name;
        double initial_value;
        double min;
        double max;
        uint8_t required;
        double scaling_factor;
        double chance;
    } CItemAttribute;

    typedef struct {
        char* name;
        CItemAttribute* attributes;
        uint32_t attributes_count;
    } CAffix;

    typedef struct {
        char* name;
        char* quality;
        char* item_type;
        char* subtype;
        CAffix prefix;
        CAffix suffix;
        CItemAttribute* attributes;
        uint32_t attributes_count;
    } CItem;

    // Memory management
    PraedaGeneratorHandle* praeda_generator_new(void);
    void praeda_generator_free(PraedaGeneratorHandle* handle);
    void praeda_string_free(char* ptr);
    void praeda_error_free(char* ptr);
    void praeda_item_array_free(CItemArrayHandle* handle);

    // Configuration
    int praeda_generator_load_toml(
        PraedaGeneratorHandle* handle,
        const char* toml_str,
        char** error_out
    );

    // Programmatic configuration
    int praeda_generator_set_quality_data(
        PraedaGeneratorHandle* handle,
        const char* quality_name,
        int weight
    );

    int praeda_generator_set_item_type(
        PraedaGeneratorHandle* handle,
        const char* type_name,
        int weight
    );

    int praeda_generator_set_item_subtype(
        PraedaGeneratorHandle* handle,
        const char* type_name,
        const char* subtype_name,
        int weight
    );

    int praeda_generator_set_attribute(
        PraedaGeneratorHandle* handle,
        const char* type_name,
        const char* subtype_name,
        const char* attr_name,
        double initial_value,
        double min_value,
        double max_value,
        int required
    );

    int praeda_generator_set_item_names(
        PraedaGeneratorHandle* handle,
        const char* type_name,
        const char* subtype_name,
        const char** names,
        uint32_t names_count
    );

    // Loot generation
    CItemArrayHandle* praeda_generator_generate_loot(
        PraedaGeneratorHandle* handle,
        uint32_t number_of_items,
        double base_level,
        double level_variance,
        double affix_chance,
        uint8_t linear,
        double scaling_factor,
        char** error_out
    );

    // Item array access
    uint32_t praeda_item_array_count(const CItemArrayHandle* handle);
    const CItem* praeda_item_array_get(const CItemArrayHandle* handle, uint32_t index);

    // Queries
    int praeda_generator_has_quality(const PraedaGeneratorHandle* handle, const char* quality);
    char* praeda_version(void);
}

// ============================================================================
// C++ Wrappers
// ============================================================================

namespace praeda {

/// RAII wrapper for C strings
class CStringWrapper {
public:
    explicit CStringWrapper(char* ptr) : ptr_(ptr) {}
    ~CStringWrapper() {
        if (ptr_) {
            praeda_error_free(ptr_);
        }
    }

    CStringWrapper(const CStringWrapper&) = delete;
    CStringWrapper& operator=(const CStringWrapper&) = delete;

    CStringWrapper(CStringWrapper&& other) noexcept : ptr_(other.release()) {}
    CStringWrapper& operator=(CStringWrapper&& other) noexcept {
        if (ptr_) praeda_error_free(ptr_);
        ptr_ = other.release();
        return *this;
    }

    const char* c_str() const { return ptr_ ? ptr_ : ""; }
    std::string str() const { return std::string(ptr_ ? ptr_ : ""); }

    char* release() {
        char* temp = ptr_;
        ptr_ = nullptr;
        return temp;
    }

private:
    char* ptr_;
};

/// Exception thrown by Praeda
class Exception : public std::runtime_error {
public:
    explicit Exception(const std::string& what) : std::runtime_error(what) {}
};

/// Item attribute - mirrors Rust ItemAttribute struct
class ItemAttribute {
public:
    ItemAttribute() : initial_value(0.0), min(0.0), max(0.0), required(false),
                      scaling_factor(1.0), chance(0.0) {}

    ItemAttribute(const std::string& n, double iv, double min_val, double max_val, bool req)
        : name(n), initial_value(iv), min(min_val), max(max_val), required(req),
          scaling_factor(1.0), chance(0.0) {}

    std::string name;
    double initial_value;
    double min;
    double max;
    bool required;
    double scaling_factor;
    double chance;

    static ItemAttribute from_c(const CItemAttribute& c_attr) {
        ItemAttribute attr;
        attr.name = std::string(c_attr.name ? c_attr.name : "");
        attr.initial_value = c_attr.initial_value;
        attr.min = c_attr.min;
        attr.max = c_attr.max;
        attr.required = c_attr.required != 0;
        attr.scaling_factor = c_attr.scaling_factor;
        attr.chance = c_attr.chance;
        return attr;
    }
};

/// Affix (prefix or suffix) - mirrors Rust Affix struct
class Affix {
public:
    Affix() = default;
    Affix(const std::string& n, const std::vector<ItemAttribute>& attrs)
        : name(n), attributes(attrs) {}

    std::string name;
    std::vector<ItemAttribute> attributes;

    static Affix from_c(const CAffix& c_affix) {
        Affix affix;
        affix.name = std::string(c_affix.name ? c_affix.name : "");

        for (uint32_t i = 0; i < c_affix.attributes_count; ++i) {
            affix.attributes.push_back(ItemAttribute::from_c(c_affix.attributes[i]));
        }

        return affix;
    }
};

/// Generated item - mirrors Rust Item struct
class Item {
public:
    Item() = default;

    Item(const std::string& n, const std::string& q, const std::string& t,
         const std::string& st, const Affix& pre, const Affix& suf,
         const std::map<std::string, ItemAttribute>& attrs)
        : name(n), quality(q), type(t), subtype(st),
          prefix(pre), suffix(suf), attributes(attrs) {}

    std::string name;
    std::string quality;
    std::string type;
    std::string subtype;
    Affix prefix;
    Affix suffix;
    std::map<std::string, ItemAttribute> attributes;

private:
    friend class Generator;

    static Item from_c(const CItem& c_item) {
        Item item;
        item.name = std::string(c_item.name ? c_item.name : "");
        item.quality = std::string(c_item.quality ? c_item.quality : "");
        item.type = std::string(c_item.item_type ? c_item.item_type : "");
        item.subtype = std::string(c_item.subtype ? c_item.subtype : "");

        item.prefix = Affix::from_c(c_item.prefix);
        item.suffix = Affix::from_c(c_item.suffix);

        for (uint32_t i = 0; i < c_item.attributes_count; ++i) {
            const CItemAttribute& c_attr = c_item.attributes[i];
            ItemAttribute attr = ItemAttribute::from_c(c_attr);
            item.attributes[attr.name] = attr;
        }

        return item;
    }
};

/// Loot generation options - mirrors Rust GeneratorOptions struct
struct GenerationOptions {
    uint32_t number_of_items = 1;
    double base_level = 1.0;
    double level_variance = 1.0;
    double affix_chance = 0.25;
    bool linear = true;
    double scaling_factor = 1.0;
};

/// Main Praeda generator class
class Generator {
public:
    /// Create a new Praeda generator
    static std::unique_ptr<Generator> create() {
        PraedaGeneratorHandle* handle = praeda_generator_new();
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
    void load_toml_string(const std::string& toml_content) {
        char* error = nullptr;
        int result = praeda_generator_load_toml(handle_, toml_content.c_str(), &error);
        if (result != 0) {
            if (error) {
                CStringWrapper error_wrapper(error);
                throw Exception(error_wrapper.str());
            }
            throw Exception("Failed to load TOML");
        }
    }

    /// Set quality tier data
    void set_quality_data(const std::string& quality_name, int weight) {
        int result = praeda_generator_set_quality_data(handle_, quality_name.c_str(), weight);
        if (result != 0) {
            throw Exception("Failed to set quality data");
        }
    }

    /// Set item type with weight
    void set_item_type(const std::string& type_name, int weight) {
        int result = praeda_generator_set_item_type(handle_, type_name.c_str(), weight);
        if (result != 0) {
            throw Exception("Failed to set item type");
        }
    }

    /// Set item subtype with weight
    void set_item_subtype(const std::string& type_name, const std::string& subtype_name, int weight) {
        int result = praeda_generator_set_item_subtype(
            handle_,
            type_name.c_str(),
            subtype_name.c_str(),
            weight
        );
        if (result != 0) {
            throw Exception("Failed to set item subtype");
        }
    }

    /// Set attribute for an item type/subtype
    void set_attribute(const std::string& type_name, const std::string& subtype_name,
                       const ItemAttribute& attribute) {
        int result = praeda_generator_set_attribute(
            handle_,
            type_name.c_str(),
            subtype_name.c_str(),
            attribute.name.c_str(),
            attribute.initial_value,
            attribute.min,
            attribute.max,
            attribute.required ? 1 : 0
        );
        if (result != 0) {
            throw Exception("Failed to set attribute");
        }
    }

    /// Set item names for a type/subtype combination
    void set_item_names(const std::string& type_name, const std::string& subtype_name,
                        const std::vector<std::string>& names) {
        std::vector<const char*> c_names;
        for (const auto& name : names) {
            c_names.push_back(name.c_str());
        }

        int result = praeda_generator_set_item_names(
            handle_,
            type_name.c_str(),
            subtype_name.c_str(),
            c_names.data(),
            static_cast<uint32_t>(c_names.size())
        );
        if (result != 0) {
            throw Exception("Failed to set item names");
        }
    }

    /// Generate loot items
    /// Returns a vector of native Item objects.
    std::vector<Item> generate_loot(const GenerationOptions& options) {
        char* error = nullptr;
        CItemArrayHandle* array_handle = praeda_generator_generate_loot(
            handle_,
            options.number_of_items,
            options.base_level,
            options.level_variance,
            options.affix_chance,
            options.linear ? 1 : 0,
            options.scaling_factor,
            &error
        );

        if (!array_handle) {
            if (error) {
                CStringWrapper error_wrapper(error);
                throw Exception(error_wrapper.str());
            }
            throw Exception("Failed to generate loot");
        }

        std::vector<Item> items;
        uint32_t count = praeda_item_array_count(array_handle);

        for (uint32_t i = 0; i < count; ++i) {
            const CItem* c_item = praeda_item_array_get(array_handle, i);
            if (c_item) {
                items.push_back(Item::from_c(*c_item));
            }
        }

        praeda_item_array_free(array_handle);
        return items;
    }

    /// Check if a quality exists
    bool has_quality(const std::string& quality) {
        int result = praeda_generator_has_quality(handle_, quality.c_str());
        if (result < 0) {
            throw Exception("Error checking quality");
        }
        return result == 1;
    }

    /// Get generator info (version string)
    std::string info() {
        char* v = praeda_version();
        CStringWrapper wrapper(v);
        return wrapper.str();
    }

    /// Get the underlying FFI handle (advanced usage only)
    PraedaGeneratorHandle* native_handle() const { return handle_; }

private:
    PraedaGeneratorHandle* handle_;

    explicit Generator(PraedaGeneratorHandle* handle) : handle_(handle) {}

    PraedaGeneratorHandle* release() {
        PraedaGeneratorHandle* temp = handle_;
        handle_ = nullptr;
        return temp;
    }
};

/// Get Praeda library version
inline std::string version() {
    char* v = praeda_version();
    CStringWrapper wrapper(v);
    return wrapper.str();
}

} // namespace praeda
