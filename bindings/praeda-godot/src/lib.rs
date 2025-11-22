use godot::prelude::*;
use praeda::{PraedaGenerator, GeneratorOptions, GeneratorOverrides, ItemAttribute};

struct PraedaExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PraedaExtension {}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct PraedaGodotGenerator {
    inner: PraedaGenerator,
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for PraedaGodotGenerator {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            inner: PraedaGenerator::new(),
            base,
        }
    }
}

#[godot_api]
impl PraedaGodotGenerator {
    #[func]
    fn set_quality_data(&mut self, name: String, weight: i32) -> bool {
        self.inner.set_quality_data(&name, weight);
        true
    }

    #[func]
    fn set_item_type(&mut self, name: String, weight: i32) -> bool {
        self.inner.set_item_type(&name, weight);
        true
    }

    #[func]
    fn set_item_subtype(&mut self, type_name: String, subtype_name: String, weight: i32) -> bool {
        self.inner.set_item_subtype(&type_name, &subtype_name, weight);
        true
    }

    #[func]
    #[allow(clippy::too_many_arguments)]
    fn set_attribute(
        &mut self, 
        type_name: String, 
        subtype_name: String, 
        attr_name: String, 
        initial_value: f64, 
        min: f64, 
        max: f64, 
        required: bool
    ) -> bool {
        let attr = ItemAttribute {
            name: attr_name,
            initial_value,
            min,
            max,
            required,
            scaling_factor: 1.0, // Default
            chance: 1.0,         // Default
        };
        
        // Handle empty strings as "all" or "none" depending on logic, 
        // but here we pass them through as the Rust API expects specific matching
        let t_name = if type_name.is_empty() { "" } else { &type_name };
        let s_name = if subtype_name.is_empty() { "" } else { &subtype_name };
        
        self.inner.set_attribute(t_name, s_name, attr);
        true
    }

    #[func]
    fn set_item_names(&mut self, type_name: String, subtype_name: String, names: Array<GString>) -> bool {
        let rust_names: Vec<String> = names.iter_shared().map(|s| s.to_string()).collect();
        // Convert Vec<String> to Vec<&str> for the API
        let name_refs: Vec<&str> = rust_names.iter().map(|s| s.as_str()).collect();
        self.inner.set_item(&type_name, &subtype_name, name_refs);
        true
    }

    #[func]
    fn generate_loot(&mut self, options: Dictionary) -> Array<Dictionary> {
        let opts = GeneratorOptions {
            number_of_items: options.get("number_of_items").map(|v| v.to::<u32>()).unwrap_or(1),
            base_level: options.get("base_level").map(|v| v.to::<f64>()).unwrap_or(1.0),
            level_variance: options.get("level_variance").map(|v| v.to::<f64>()).unwrap_or(0.0),
            affix_chance: options.get("affix_chance").map(|v| v.to::<f64>()).unwrap_or(0.5),
            linear: options.get("linear").map(|v| v.to::<bool>()).unwrap_or(true),
            scaling_factor: options.get("scaling_factor").map(|v| v.to::<f64>()).unwrap_or(1.0),
        };

        let overrides = GeneratorOverrides::empty(); // Simplified for this example
        
        let mut result_array = Array::new();

        if let Ok(items) = self.inner.generate_loot(&opts, &overrides, "main") {
            for item in items {
                let mut dict = Dictionary::new();
                dict.set("name", item.name);
                dict.set("quality", item.quality);
                dict.set("type", item.item_type);
                dict.set("subtype", item.subtype);
                
                // Convert attributes to dictionary
                let mut attrs_dict = Dictionary::new();
                for (name, attr) in item.attributes {
                    attrs_dict.set(name, attr.initial_value);
                }
                dict.set("attributes", attrs_dict);
                
                result_array.push(&dict);
            }
        }

        result_array
    }
}
