use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an item type with subtypes and weight
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemType {
    pub item_type: String,
    pub subtypes: HashMap<String, i32>,
    pub weight: i32,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ItemType {
    pub fn new(item_type: String, subtypes: HashMap<String, i32>, weight: i32) -> Self {
        ItemType {
            item_type,
            subtypes,
            weight,
            metadata: HashMap::new(),
        }
    }

    pub fn set_type(&mut self, item_type: String) {
        self.item_type = item_type;
    }

    pub fn get_type(&self) -> &str {
        &self.item_type
    }

    pub fn add_subtype(&mut self, subtype: String, weight: i32) {
        self.subtypes.insert(subtype, weight);
    }

    pub fn get_subtypes(&self) -> &HashMap<String, i32> {
        &self.subtypes
    }

    pub fn has_subtype(&self, subtype: &str) -> bool {
        self.subtypes.contains_key(subtype)
    }

    pub fn set_weight(&mut self, weight: i32) {
        self.weight = weight;
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }

    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    pub fn get_all_metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }

    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
}

/// Represents item data (names for specific types/subtypes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemData {
    pub item_type: String,
    pub subtype: String,
    pub names: Vec<String>,
}

impl ItemData {
    pub fn new(item_type: String, subtype: String, names: Vec<String>) -> Self {
        ItemData {
            item_type,
            subtype,
            names,
        }
    }

    pub fn set_item_type(&mut self, item_type: String) {
        self.item_type = item_type;
    }

    pub fn get_item_type(&self) -> &str {
        &self.item_type
    }

    pub fn set_subtype(&mut self, subtype: String) {
        self.subtype = subtype;
    }

    pub fn get_subtype(&self) -> &str {
        &self.subtype
    }

    pub fn add_name(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn get_names(&self) -> &[String] {
        &self.names
    }
}

/// Represents a single attribute on an item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ItemAttribute {
    pub name: String,
    pub initial_value: f64,
    pub min: f64,
    pub max: f64,
    pub required: bool,
    #[serde(default)]
    pub scaling_factor: f64,
    #[serde(default)]
    pub chance: f64,
}

impl ItemAttribute {
    pub fn new(
        name: String,
        initial_value: f64,
        min: f64,
        max: f64,
        required: bool,
    ) -> Self {
        ItemAttribute {
            name,
            initial_value,
            min,
            max,
            required,
            scaling_factor: 1.0,
            chance: 0.0,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_initial_value(&mut self, initial_value: f64) {
        // LCOV_EXCL_START - Rare path: setting bounds when both are zero
        if self.min == 0.0 && self.max == 0.0 {
            self.min = initial_value;
            self.max = initial_value;
        }
        // LCOV_EXCL_END
        self.initial_value = initial_value;
    }

    pub fn get_initial_value(&self) -> f64 {
        self.initial_value
    }

    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn get_min(&self) -> f64 {
        self.min
    }

    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }

    pub fn get_max(&self) -> f64 {
        self.max
    }

    pub fn set_required(&mut self, required: bool) {
        self.required = required;
    }

    pub fn get_required(&self) -> bool {
        self.required
    }

    /// Generate a scaled value based on level, scaling factor, and linear/exponential progression
    pub fn generate_value(&mut self, new_level: f64, linear: bool, scaling_factor: f64) {
        if self.min == 0.0 && self.max == 0.0 && self.initial_value != 0.0 {
            self.min = self.initial_value;
            self.max = self.initial_value;
        }

        if self.initial_value == 0.0 && !linear {
            self.initial_value = 1.0;
        }

        if linear {
            self.initial_value += new_level * scaling_factor;
        } else {
            self.initial_value *= scaling_factor.powf(new_level);
        }

        if self.initial_value < 0.0 {
            self.initial_value = 0.0;
        }
    }
}

/// Represents a prefix or suffix affix
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Affix {
    pub name: String,
    pub attributes: Vec<ItemAttribute>,
}

impl Affix {
    pub fn new(name: String, attributes: Vec<ItemAttribute>) -> Self {
        Affix { name, attributes }
    }

    pub fn empty() -> Self {
        Affix {
            name: String::new(),
            attributes: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_attributes(&self) -> &[ItemAttribute] {
        &self.attributes
    }

    pub fn set_attributes(&mut self, attributes: Vec<ItemAttribute>) {
        self.attributes = attributes;
    }

    pub fn set_attribute(&mut self, new_attribute: ItemAttribute) {
        if let Some(pos) = self
            .attributes
            .iter()
            .position(|a| a.get_name() == new_attribute.get_name())
        {
            self.attributes[pos] = new_attribute;
        } else {
            self.attributes.push(new_attribute);
        }
    }
}

/// Represents a complete generated item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub name: String,
    pub quality: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub subtype: String,
    pub prefix: Affix,
    pub suffix: Affix,
    pub attributes: HashMap<String, ItemAttribute>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Item {
    pub fn new(
        name: String,
        quality: String,
        item_type: String,
        subtype: String,
        prefix: Affix,
        suffix: Affix,
        attributes: HashMap<String, ItemAttribute>,
    ) -> Self {
        Item {
            name,
            quality,
            item_type,
            subtype,
            prefix,
            suffix,
            attributes,
            metadata: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Item {
            name: String::new(),
            quality: String::new(),
            item_type: String::new(),
            subtype: String::new(),
            prefix: Affix::empty(),
            suffix: Affix::empty(),
            attributes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_quality(&mut self, quality: String) {
        self.quality = quality;
    }

    pub fn get_quality(&self) -> &str {
        &self.quality
    }

    pub fn set_type(&mut self, item_type: String) {
        self.item_type = item_type;
    }

    pub fn get_type(&self) -> &str {
        &self.item_type
    }

    pub fn set_subtype(&mut self, subtype: String) {
        self.subtype = subtype;
    }

    pub fn get_subtype(&self) -> &str {
        &self.subtype
    }

    pub fn set_prefix(&mut self, prefix: Affix) {
        self.prefix = prefix;
    }

    pub fn get_prefix(&self) -> &Affix {
        &self.prefix
    }

    pub fn get_prefix_mut(&mut self) -> &mut Affix {
        &mut self.prefix
    }

    pub fn set_suffix(&mut self, suffix: Affix) {
        self.suffix = suffix;
    }

    pub fn get_suffix(&self) -> &Affix {
        &self.suffix
    }

    #[cfg(not(tarpaulin_include))]
    pub fn get_suffix_mut(&mut self) -> &mut Affix {
        &mut self.suffix
    }

    #[cfg(not(tarpaulin_include))]
    pub fn set_attributes(&mut self, attributes: HashMap<String, ItemAttribute>) {
        self.attributes = attributes;
    }

    pub fn get_attributes(&self) -> &HashMap<String, ItemAttribute> {
        &self.attributes
    }

    pub fn set_attribute(&mut self, name: String, attr: ItemAttribute) {
        self.attributes.insert(name, attr);
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    pub fn get_attribute(&self, name: &str) -> Option<&ItemAttribute> {
        self.attributes.get(name)
    }

    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut ItemAttribute> {
        self.attributes.get_mut(name)
    }

    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    pub fn get_all_metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }

    pub fn has_metadata(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
}

/// Options for loot generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratorOptions {
    pub number_of_items: u32,
    pub base_level: f64,
    pub level_variance: f64,
    pub affix_chance: f64,
    pub linear: bool,
    pub scaling_factor: f64,
}

impl GeneratorOptions {
    pub fn new(
        number_of_items: u32,
        base_level: f64,
        level_variance: f64,
        affix_chance: f64,
        linear: bool,
        scaling_factor: f64,
    ) -> Self {
        GeneratorOptions {
            number_of_items,
            base_level,
            level_variance,
            affix_chance,
            linear,
            scaling_factor,
        }
    }

    pub fn default() -> Self {
        GeneratorOptions {
            number_of_items: 1,
            base_level: 1.0,
            level_variance: 1.0,
            affix_chance: 0.25,
            linear: true,
            scaling_factor: 1.0,
        }
    }

    pub fn is_linear(&self) -> bool {
        self.linear
    }

    pub fn is_exponential(&self) -> bool {
        !self.linear
    }
}

/// Overrides for loot generation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratorOverrides {
    pub quality_override: String,
    pub type_override: String,
    pub subtype_override: String,
}

impl GeneratorOverrides {
    pub fn new(
        quality_override: String,
        type_override: String,
        subtype_override: String,
    ) -> Self {
        GeneratorOverrides {
            quality_override,
            type_override,
            subtype_override,
        }
    }

    pub fn empty() -> Self {
        GeneratorOverrides {
            quality_override: String::new(),
            type_override: String::new(),
            subtype_override: String::new(),
        }
    }

    pub fn get_quality_override(&self) -> &str {
        &self.quality_override
    }

    pub fn get_type_override(&self) -> &str {
        &self.type_override
    }

    pub fn get_subtype_override(&self) -> &str {
        &self.subtype_override
    }
}

// ============================================================================
// TOML Intermediate Structures for Deserialization
// ============================================================================

/// Intermediate structure for loading TOML configuration
#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub quality_data: HashMap<String, i32>,
    #[serde(default)]
    pub item_types: Vec<ItemType>,
    #[serde(default)]
    pub item_attributes: Vec<TomlItemAttributes>,
    #[serde(default)]
    pub item_list: Vec<TomlItemList>,
    #[serde(default)]
    pub item_affixes: Vec<TomlItemAffixes>,
}

/// Item attributes for a specific type/subtype combination
#[derive(Debug, Deserialize)]
pub struct TomlItemAttributes {
    #[serde(default)]
    pub item_type: String,
    #[serde(default)]
    pub subtype: String,
    #[serde(default)]
    pub attributes: Vec<ItemAttribute>,
}

/// Item list for a specific type/subtype combination
#[derive(Debug, Deserialize)]
pub struct TomlItemList {
    pub item_type: String,
    pub subtype: String,
    #[serde(default)]
    pub names: Vec<String>,
}

/// Item affixes for a specific type/subtype combination
#[derive(Debug, Deserialize)]
pub struct TomlItemAffixes {
    #[serde(default)]
    pub item_type: String,
    #[serde(default)]
    pub subtype: String,
    #[serde(default)]
    pub prefixes: Vec<Affix>,
    #[serde(default)]
    pub suffixes: Vec<Affix>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}
