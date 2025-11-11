use praeda::*;
use std::collections::HashMap; // Used in test_item_struct
use std::fs;
use std::path::Path;

/// Helper to create a basic generator with standard configuration
fn create_test_generator() -> PraedaGenerator {
    let mut gen = PraedaGenerator::new();

    // Quality tiers
    gen.set_quality_data("common".to_string(), 100);
    gen.set_quality_data("uncommon".to_string(), 60);
    gen.set_quality_data("rare".to_string(), 30);

    // Item types
    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_type("armor".to_string(), 1);

    // Subtypes
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "axe".to_string(), 1);
    gen.set_item_subtype("armor".to_string(), "head".to_string(), 1);

    // Attributes
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    gen.set_attribute(
        "armor".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "defense".to_string(),
            5.0,
            1.0,
            10.0,
            true,
        ),
    );

    // Item names
    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["longsword".to_string(), "shortsword".to_string()],
    );
    gen.set_item(
        "weapon".to_string(),
        "axe".to_string(),
        vec!["battleaxe".to_string()],
    );
    gen.set_item(
        "armor".to_string(),
        "head".to_string(),
        vec!["helm".to_string(), "crown".to_string()],
    );

    // Affixes
    gen.set_affix("weapon".to_string(), "".to_string());
    gen.set_affix_attribute(
        "weapon".to_string(),
        "".to_string(),
        true,
        "sharp".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            5.0,
            0.0,
            0.0,
            false,
        ),
    );

    gen.set_affix_attribute(
        "weapon".to_string(),
        "".to_string(),
        false,
        "of fire".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            3.0,
            0.0,
            0.0,
            false,
        ),
    );

    gen
}

#[test]
fn test_generator_creation() {
    let gen = PraedaGenerator::new();
    assert_eq!(gen.get_quality_data().len(), 0);
    assert_eq!(gen.get_item_types().len(), 0);
}

#[test]
fn test_set_quality_data() {
    let mut gen = PraedaGenerator::new();
    gen.set_quality_data("common".to_string(), 100);
    gen.set_quality_data("rare".to_string(), 10);

    assert!(gen.has_quality("common"));
    assert!(gen.has_quality("rare"));
    assert!(!gen.has_quality("epic"));
}

#[test]
fn test_set_item_type() {
    let mut gen = PraedaGenerator::new();
    gen.set_item_type("weapon".to_string(), 50);
    gen.set_item_type("armor".to_string(), 50);

    assert!(gen.has_item_type("weapon"));
    assert!(gen.has_item_type("armor"));
    assert!(!gen.has_item_type("shield"));
}

#[test]
fn test_set_item_subtype() {
    let mut gen = PraedaGenerator::new();
    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 50);

    assert!(gen.has_item_subtype("weapon", "sword"));
    assert!(!gen.has_item_subtype("weapon", "bow"));
}

#[test]
fn test_empty_string_overrides_always_match() {
    let gen = PraedaGenerator::new();
    assert!(gen.has_quality(""));
    assert!(gen.has_item_type(""));
    assert!(gen.has_item_subtype("", ""));
}

#[test]
fn test_single_item_generation() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 1,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.5,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "test")?;

    assert_eq!(items.len(), 1);
    let item = &items[0];

    // Verify item has required fields
    assert!(!item.get_name().is_empty());
    assert!(!item.get_quality().is_empty());
    assert!(!item.get_type().is_empty());
    assert!(item.has_attribute("level"));

    Ok(())
}

#[test]
fn test_multiple_items_generation() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 100,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "bulk")?;

    assert_eq!(items.len(), 100);

    // Verify all items are valid
    for item in items {
        assert!(!item.get_name().is_empty());
        assert!(!item.get_quality().is_empty());
    }

    Ok(())
}

#[test]
fn test_quality_override() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions::default();
    let overrides = GeneratorOverrides::new(
        "rare".to_string(),
        "".to_string(),
        "".to_string(),
    );

    let items = gen.generate_loot(&options, &overrides, "quality_override")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_quality(), "rare");

    Ok(())
}

#[test]
fn test_type_override() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions::default();
    let overrides = GeneratorOverrides::new(
        "".to_string(),
        "weapon".to_string(),
        "".to_string(),
    );

    let items = gen.generate_loot(&options, &overrides, "type_override")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_type(), "weapon");

    Ok(())
}

#[test]
fn test_subtype_override() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions::default();
    let overrides = GeneratorOverrides::new(
        "".to_string(),
        "weapon".to_string(),
        "sword".to_string(),
    );

    let items = gen.generate_loot(&options, &overrides, "subtype_override")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_subtype(), "sword");

    Ok(())
}

#[test]
fn test_linear_vs_exponential_scaling() -> Result<()> {
    let mut gen1 = create_test_generator();
    let mut gen2 = create_test_generator();

    let linear_opts = GeneratorOptions {
        number_of_items: 10,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 1.0, // Set to 1.0 to ensure optional attributes are applied
        linear: true,
        scaling_factor: 1.5,
    };

    let exp_opts = GeneratorOptions {
        number_of_items: 10,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 1.0, // Set to 1.0 to ensure optional attributes are applied
        linear: false,
        scaling_factor: 1.5,
    };

    let linear_items = gen1.generate_loot(&linear_opts, &GeneratorOverrides::empty(), "linear")?;
    let exp_items = gen2.generate_loot(&exp_opts, &GeneratorOverrides::empty(), "exp")?;

    // Both should generate items
    assert_eq!(linear_items.len(), 10);
    assert_eq!(exp_items.len(), 10);

    // Both should have level attribute (required)
    assert!(linear_items[0].has_attribute("level"));
    assert!(exp_items[0].has_attribute("level"));

    Ok(())
}

#[test]
fn test_json_serialization() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 1,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "json_test")?;
    let json_str = serde_json::to_string(&items)?;

    // Should be valid JSON
    let _: Vec<Item> = serde_json::from_str(&json_str)?;

    Ok(())
}

#[test]
fn test_generator_serialization() -> Result<()> {
    // Create minimal generator for serialization testing
    let mut gen = PraedaGenerator::new();
    gen.set_quality_data("common".to_string(), 100);
    gen.set_quality_data("rare".to_string(), 10);
    gen.set_item_type("weapon".to_string(), 1);

    let json = gen.save_data()?;

    // Should be valid JSON
    let _: serde_json::Value = serde_json::from_str(&json)?;

    // Should be able to load it back
    let mut gen2 = PraedaGenerator::new();
    gen2.load_data(&json)?;

    // Verify data matches
    assert_eq!(gen.get_quality_data().len(), gen2.get_quality_data().len());
    assert_eq!(gen.get_item_types().len(), gen2.get_item_types().len());

    Ok(())
}

#[test]
fn test_affixes_applied_to_items() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 50,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 1.0, // Always apply affixes
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "affix_test")?;

    let mut has_prefix = false;
    let mut has_suffix = false;

    for item in items {
        if !item.get_prefix().get_name().is_empty() {
            has_prefix = true;
        }
        if !item.get_suffix().get_name().is_empty() {
            has_suffix = true;
        }
    }

    // With high affix chance and enough items, should see some affixes
    assert!(has_prefix || has_suffix);

    Ok(())
}

#[test]
fn test_get_prefixes_and_suffixes() {
    let gen = create_test_generator();

    let prefixes = gen.get_prefixes("weapon", "");
    let suffixes = gen.get_suffixes("weapon", "");

    assert_eq!(prefixes.len(), 1);
    assert_eq!(suffixes.len(), 1);
    assert_eq!(prefixes[0].get_name(), "sharp");
    assert_eq!(suffixes[0].get_name(), "of fire");
}

#[test]
fn test_item_attribute_struct() {
    let mut attr = ItemAttribute::new(
        "health".to_string(),
        100.0,
        0.0,
        200.0,
        true,
    );

    assert_eq!(attr.get_name(), "health");
    assert_eq!(attr.get_initial_value(), 100.0);
    assert!(attr.get_required());

    attr.set_initial_value(150.0);
    assert_eq!(attr.get_initial_value(), 150.0);
}

#[test]
fn test_item_struct() {
    let item = Item::new(
        "sword".to_string(),
        "rare".to_string(),
        "weapon".to_string(),
        "sword".to_string(),
        Affix::empty(),
        Affix::empty(),
        HashMap::new(),
    );

    assert_eq!(item.get_name(), "sword");
    assert_eq!(item.get_quality(), "rare");
    assert_eq!(item.get_type(), "weapon");
    assert_eq!(item.get_subtype(), "sword");
}

#[test]
fn test_affix_struct() {
    let attr = ItemAttribute::new(
        "damage".to_string(),
        10.0,
        0.0,
        0.0,
        false,
    );

    let affix = Affix::new("sharp".to_string(), vec![attr]);

    assert_eq!(affix.get_name(), "sharp");
    assert_eq!(affix.get_attributes().len(), 1);
    assert_eq!(affix.get_attributes()[0].get_name(), "damage");
}

#[test]
fn test_generator_options_defaults() {
    let opts = GeneratorOptions::default();

    assert_eq!(opts.number_of_items, 1);
    assert_eq!(opts.base_level, 1.0);
    assert_eq!(opts.level_variance, 1.0);
    assert_eq!(opts.affix_chance, 0.25);
    assert!(opts.linear);
    assert_eq!(opts.scaling_factor, 1.0);
}

#[test]
fn test_generator_overrides_empty() {
    let overrides = GeneratorOverrides::empty();

    assert_eq!(overrides.get_quality_override(), "");
    assert_eq!(overrides.get_type_override(), "");
    assert_eq!(overrides.get_subtype_override(), "");
}

#[test]
fn test_loot_retrieval() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions::default();
    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "retrieval_test")?;

    let retrieved = gen.get_loot("retrieval_test");
    assert_eq!(retrieved.len(), items.len());

    let json = gen.get_loot_json("retrieval_test")?;
    assert!(!json.is_empty());

    Ok(())
}

#[test]
fn test_nonexistent_loot_retrieval() {
    let gen = PraedaGenerator::new();

    let items = gen.get_loot("nonexistent");
    assert_eq!(items.len(), 0);
}

#[test]
fn test_has_attribute() {
    let gen = create_test_generator();

    assert!(gen.has_attribute("weapon", "", "damage"));
    assert!(gen.has_attribute("armor", "", "defense"));
    assert!(!gen.has_attribute("weapon", "", "nonexistent"));
}

#[test]
fn test_empty_quality_data_handles_gracefully() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Should fail gracefully when trying to generate with no qualities
    let options = GeneratorOptions::default();
    let result = gen.generate_loot(&options, &GeneratorOverrides::empty(), "empty");

    // It should fail since there's no quality data
    assert!(result.is_err());

    Ok(())
}


#[test]
fn test_quality_distribution() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Setup with very unbalanced weights
    gen.set_quality_data("common".to_string(), 1000);
    gen.set_quality_data("rare".to_string(), 1);

    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );
    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["sword".to_string()],
    );

    let options = GeneratorOptions {
        number_of_items: 100,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "distribution")?;

    let common_count = items.iter().filter(|i| i.get_quality() == "common").count();
    let rare_count = items.iter().filter(|i| i.get_quality() == "rare").count();

    // Most items should be common (1000:1 ratio)
    assert!(common_count > rare_count * 5);

    Ok(())
}

#[test]
fn test_quality_weights_respect_ratios() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Setup with balanced weights: 50% common, 30% uncommon, 20% rare
    gen.set_quality_data("common".to_string(), 50);
    gen.set_quality_data("uncommon".to_string(), 30);
    gen.set_quality_data("rare".to_string(), 20);

    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );
    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["sword".to_string()],
    );

    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "weight_test")?;

    let common_count = items.iter().filter(|i| i.get_quality() == "common").count() as f64;
    let uncommon_count = items.iter().filter(|i| i.get_quality() == "uncommon").count() as f64;
    let rare_count = items.iter().filter(|i| i.get_quality() == "rare").count() as f64;
    let total = items.len() as f64;

    let common_pct = common_count / total;
    let uncommon_pct = uncommon_count / total;
    let rare_pct = rare_count / total;

    // Allow 10% deviation from expected percentages
    assert!((common_pct - 0.50).abs() < 0.10, "common: expected 50%, got {}", common_pct * 100.0);
    assert!((uncommon_pct - 0.30).abs() < 0.10, "uncommon: expected 30%, got {}", uncommon_pct * 100.0);
    assert!((rare_pct - 0.20).abs() < 0.10, "rare: expected 20%, got {}", rare_pct * 100.0);

    Ok(())
}

#[test]
fn test_item_type_weights() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Setup with 2:1 weapon to armor ratio
    gen.set_quality_data("common".to_string(), 100);
    gen.set_item_type("weapon".to_string(), 2);
    gen.set_item_type("armor".to_string(), 1);

    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_item_subtype("armor".to_string(), "head".to_string(), 1);

    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["sword".to_string()],
    );
    gen.set_item(
        "armor".to_string(),
        "head".to_string(),
        vec!["helm".to_string()],
    );

    let options = GeneratorOptions {
        number_of_items: 300,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "type_weights")?;

    let weapon_count = items.iter().filter(|i| i.get_type() == "weapon").count() as f64;
    let armor_count = items.iter().filter(|i| i.get_type() == "armor").count() as f64;
    let total = items.len() as f64;

    let weapon_pct = weapon_count / total;
    let armor_pct = armor_count / total;

    // Expect roughly 2:1 ratio (66% weapons, 33% armor)
    // Allow 15% deviation
    assert!(weapon_pct > 0.51 && weapon_pct < 0.81, "weapons: expected ~66%, got {}", weapon_pct * 100.0);
    assert!(armor_pct > 0.19 && armor_pct < 0.49, "armor: expected ~33%, got {}", armor_pct * 100.0);

    Ok(())
}

#[test]
fn test_subtype_weights() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Setup with 3:1 ratio of one-handed to two-handed
    gen.set_quality_data("common".to_string(), 100);
    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "one-handed".to_string(), 3);
    gen.set_item_subtype("weapon".to_string(), "two-handed".to_string(), 1);

    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    gen.set_item(
        "weapon".to_string(),
        "one-handed".to_string(),
        vec!["sword".to_string()],
    );
    gen.set_item(
        "weapon".to_string(),
        "two-handed".to_string(),
        vec!["claymore".to_string()],
    );

    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "subtype_weights")?;

    let one_handed_count = items.iter().filter(|i| i.get_subtype() == "one-handed").count() as f64;
    let two_handed_count = items.iter().filter(|i| i.get_subtype() == "two-handed").count() as f64;
    let total = items.len() as f64;

    let one_handed_pct = one_handed_count / total;
    let two_handed_pct = two_handed_count / total;

    // Expect roughly 3:1 ratio (75% one-handed, 25% two-handed)
    // Allow 10% deviation (with 1000 items, variance should be small)
    assert!(one_handed_pct > 0.65 && one_handed_pct < 0.85, "one-handed: expected ~75%, got {}", one_handed_pct * 100.0);
    assert!(two_handed_pct > 0.15 && two_handed_pct < 0.35, "two-handed: expected ~25%, got {}", two_handed_pct * 100.0);

    Ok(())
}

/// Test 1: High variance scaling with exponential growth
/// Simulates a game with varied item levels (1-100) and exponential attribute scaling
#[test]
fn test_exponential_scaling_variance() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Setup qualities with heavy weights toward common
    gen.set_quality_data("common".to_string(), 1000);
    gen.set_quality_data("uncommon".to_string(), 300);
    gen.set_quality_data("rare".to_string(), 100);
    gen.set_quality_data("epic".to_string(), 20);
    gen.set_quality_data("legendary".to_string(), 1);

    // Multiple item types with varied weights
    gen.set_item_type("weapon".to_string(), 5);
    gen.set_item_type("armor".to_string(), 4);
    gen.set_item_type("accessory".to_string(), 1);

    // Weapon subtypes
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 3);
    gen.set_item_subtype("weapon".to_string(), "axe".to_string(), 2);
    gen.set_item_subtype("weapon".to_string(), "bow".to_string(), 1);

    // Armor subtypes
    gen.set_item_subtype("armor".to_string(), "chest".to_string(), 2);
    gen.set_item_subtype("armor".to_string(), "legs".to_string(), 2);
    gen.set_item_subtype("armor".to_string(), "head".to_string(), 1);

    // Accessory subtypes
    gen.set_item_subtype("accessory".to_string(), "ring".to_string(), 1);

    // Set attributes with exponential scaling
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "attack".to_string(),
            50.0,
            10.0,
            100.0,
            true,
        ),
    );

    gen.set_attribute(
        "armor".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "defense".to_string(),
            30.0,
            5.0,
            60.0,
            true,
        ),
    );

    gen.set_attribute(
        "accessory".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "magic".to_string(),
            20.0,
            5.0,
            50.0,
            true,
        ),
    );

    // Set item names
    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["longsword".to_string(), "shortsword".to_string(), "claymore".to_string()],
    );
    gen.set_item(
        "weapon".to_string(),
        "axe".to_string(),
        vec!["war_axe".to_string(), "hand_axe".to_string()],
    );
    gen.set_item(
        "weapon".to_string(),
        "bow".to_string(),
        vec!["longbow".to_string()],
    );
    gen.set_item(
        "armor".to_string(),
        "chest".to_string(),
        vec!["plate_chest".to_string(), "leather_chest".to_string()],
    );
    gen.set_item(
        "armor".to_string(),
        "legs".to_string(),
        vec!["plate_legs".to_string(), "leather_legs".to_string()],
    );
    gen.set_item(
        "armor".to_string(),
        "head".to_string(),
        vec!["helmet".to_string()],
    );
    gen.set_item(
        "accessory".to_string(),
        "ring".to_string(),
        vec!["gold_ring".to_string(), "silver_ring".to_string()],
    );

    // Generate with high variance and exponential scaling
    let options = GeneratorOptions {
        number_of_items: 500,
        base_level: 50.0,
        level_variance: 40.0,
        affix_chance: 0.3,
        linear: false, // Exponential scaling
        scaling_factor: 1.5,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "exp_scaling")?;

    // Verify items were generated
    assert_eq!(items.len(), 500);

    // Verify all items have expected types
    let valid_types: Vec<&str> = vec!["weapon", "armor", "accessory"];
    for item in &items {
        assert!(valid_types.contains(&item.get_type().as_ref()));
    }

    // Verify quality distribution roughly matches weights (1421 total weight)
    let common_pct = items.iter().filter(|i| i.get_quality() == "common").count() as f64 / 500.0;
    assert!(common_pct > 0.60 && common_pct < 0.75, "common expected ~70%, got {}", common_pct * 100.0);

    Ok(())
}

/// Test 2: Minimal setup - single type, single subtype, single quality
/// Verifies library works with minimal configuration
#[test]
fn test_minimal_single_item_generation() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Absolute minimum setup
    gen.set_quality_data("normal".to_string(), 1);
    gen.set_item_type("tool".to_string(), 1);
    gen.set_item_subtype("tool".to_string(), "pickaxe".to_string(), 1);

    gen.set_attribute(
        "tool".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "durability".to_string(),
            50.0,
            10.0,
            100.0,
            true,
        ),
    );

    gen.set_item(
        "tool".to_string(),
        "pickaxe".to_string(),
        vec!["pickaxe".to_string()],
    );

    let options = GeneratorOptions {
        number_of_items: 10,
        base_level: 1.0,
        level_variance: 0.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "minimal")?;

    // All items should be identical (same quality, type, subtype, name)
    assert_eq!(items.len(), 10);
    for item in &items {
        assert_eq!(item.get_quality(), "normal");
        assert_eq!(item.get_type(), "tool");
        assert_eq!(item.get_subtype(), "pickaxe");
        assert_eq!(item.get_name(), "pickaxe");
    }

    Ok(())
}

/// Test 3: Extremely skewed weights (1000:1 ratio)
/// Tests that the algorithm handles extreme weight disparities
#[test]
fn test_extreme_weight_skew() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Setup with extreme skew toward common
    gen.set_quality_data("common".to_string(), 1000);
    gen.set_quality_data("legendary".to_string(), 1);

    gen.set_item_type("weapon".to_string(), 1000);
    gen.set_item_type("special".to_string(), 1);

    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_item_subtype("special".to_string(), "artifact".to_string(), 1);

    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    gen.set_attribute(
        "special".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "power".to_string(),
            100.0,
            50.0,
            150.0,
            true,
        ),
    );

    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["sword".to_string()],
    );
    gen.set_item(
        "special".to_string(),
        "artifact".to_string(),
        vec!["artifact".to_string()],
    );

    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "skew")?;

    // With 1000:1 weight, expect almost all to be the heavy weight item
    let common_count = items.iter().filter(|i| i.get_quality() == "common").count();
    let common_pct = common_count as f64 / 1000.0;

    // Should be >98% common (with 1000:1 ratio, expected rate is ~99.9%)
    assert!(common_pct > 0.98, "common expected >98%, got {}", common_pct * 100.0);

    Ok(())
}

/// Test 4: Many item types (10+) with varied weights
/// Tests performance and correctness with complex item hierarchies
#[test]
fn test_many_item_types() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    gen.set_quality_data("common".to_string(), 100);
    gen.set_quality_data("rare".to_string(), 10);

    // 10 different weapon types with varied weights
    let weapon_types = vec![
        ("sword", 50),
        ("axe", 40),
        ("mace", 30),
        ("bow", 20),
        ("staff", 15),
        ("spear", 10),
        ("dagger", 8),
        ("flail", 5),
        ("wand", 3),
        ("club", 2),
    ];

    gen.set_item_type("weapon".to_string(), 1);

    for (subtype, weight) in &weapon_types {
        gen.set_item_subtype("weapon".to_string(), subtype.to_string(), *weight);
        gen.set_item(
            "weapon".to_string(),
            subtype.to_string(),
            vec![format!("{}1", subtype), format!("{}2", subtype)],
        );
    }

    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            25.0,
            5.0,
            50.0,
            true,
        ),
    );

    let options = GeneratorOptions {
        number_of_items: 500,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.2,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "many_types")?;

    assert_eq!(items.len(), 500);

    // Verify sword is most common (weight 50 out of 183 total)
    let sword_count = items.iter().filter(|i| i.get_subtype() == "sword").count();
    let sword_pct = sword_count as f64 / 500.0;
    let expected_sword_pct = 50.0 / 183.0;

    // Allow 8% deviation
    assert!(
        (sword_pct - expected_sword_pct).abs() < 0.08,
        "sword expected ~{}%, got {}%",
        expected_sword_pct * 100.0,
        sword_pct * 100.0
    );

    // Verify rarest item exists and is rare
    let club_count = items.iter().filter(|i| i.get_subtype() == "club").count();
    let club_pct = club_count as f64 / 500.0;
    assert!(club_pct < 0.08, "club expected <8%, got {}", club_pct * 100.0);

    Ok(())
}

/// Test 5: Full RPG scenario - weapons, armor, accessories with different distributions
/// Tests realistic game loot generation
#[test]
fn test_full_rpg_loot_scenario() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    // Quality tiers following typical game distribution
    gen.set_quality_data("common".to_string(), 500);
    gen.set_quality_data("uncommon".to_string(), 250);
    gen.set_quality_data("rare".to_string(), 100);
    gen.set_quality_data("epic".to_string(), 30);
    gen.set_quality_data("legendary".to_string(), 5);

    // Item types with realistic proportions
    gen.set_item_type("weapon".to_string(), 4);
    gen.set_item_type("armor".to_string(), 3);
    gen.set_item_type("accessory".to_string(), 2);
    gen.set_item_type("consumable".to_string(), 1);

    // Weapon subtypes
    let weapon_subtypes = vec![
        ("sword", 3),
        ("axe", 2),
        ("bow", 2),
        ("staff", 1),
    ];
    for (subtype, weight) in &weapon_subtypes {
        gen.set_item_subtype("weapon".to_string(), subtype.to_string(), *weight);
        gen.set_item("weapon".to_string(), subtype.to_string(), vec![subtype.to_string()]);
    }

    // Armor subtypes
    let armor_subtypes = vec![
        ("chest", 2),
        ("legs", 2),
        ("head", 1),
        ("feet", 1),
        ("hands", 1),
    ];
    for (subtype, weight) in &armor_subtypes {
        gen.set_item_subtype("armor".to_string(), subtype.to_string(), *weight);
        gen.set_item("armor".to_string(), subtype.to_string(), vec![subtype.to_string()]);
    }

    // Accessory subtypes
    gen.set_item_subtype("accessory".to_string(), "ring".to_string(), 1);
    gen.set_item("accessory".to_string(), "ring".to_string(), vec!["ring".to_string()]);

    gen.set_item_subtype("accessory".to_string(), "amulet".to_string(), 1);
    gen.set_item("accessory".to_string(), "amulet".to_string(), vec!["amulet".to_string()]);

    // Consumable subtypes
    gen.set_item_subtype("consumable".to_string(), "potion".to_string(), 1);
    gen.set_item("consumable".to_string(), "potion".to_string(), vec!["potion".to_string()]);

    // Add attributes to all types
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            30.0,
            10.0,
            60.0,
            true,
        ),
    );

    gen.set_attribute(
        "armor".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "defense".to_string(),
            20.0,
            5.0,
            40.0,
            true,
        ),
    );

    gen.set_attribute(
        "accessory".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "bonus".to_string(),
            10.0,
            2.0,
            20.0,
            true,
        ),
    );

    gen.set_attribute(
        "consumable".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "effect".to_string(),
            5.0,
            1.0,
            10.0,
            true,
        ),
    );

    // Generate with affix chance
    let options = GeneratorOptions {
        number_of_items: 1000,
        base_level: 20.0,
        level_variance: 10.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "rpg_loot")?;

    assert_eq!(items.len(), 1000);

    // Verify distribution of item types (4:3:2:1 ratio = 40:30:20:10)
    let weapon_count = items.iter().filter(|i| i.get_type() == "weapon").count() as f64 / 1000.0;
    let armor_count = items.iter().filter(|i| i.get_type() == "armor").count() as f64 / 1000.0;
    let accessory_count = items.iter().filter(|i| i.get_type() == "accessory").count() as f64 / 1000.0;
    let consumable_count = items.iter().filter(|i| i.get_type() == "consumable").count() as f64 / 1000.0;

    // Allow 8% deviation
    assert!(weapon_count > 0.32 && weapon_count < 0.48, "weapons expected ~40%, got {}", weapon_count * 100.0);
    assert!(armor_count > 0.22 && armor_count < 0.38, "armor expected ~30%, got {}", armor_count * 100.0);
    assert!(accessory_count > 0.12 && accessory_count < 0.28, "accessories expected ~20%, got {}", accessory_count * 100.0);
    assert!(consumable_count > 0.02 && consumable_count < 0.18, "consumables expected ~10%, got {}", consumable_count * 100.0);

    // Verify all items have valid attributes
    for item in &items {
        let attrs = item.get_attributes();
        assert!(!attrs.is_empty(), "item should have attributes");
    }

    Ok(())
}

/// Test 6: Linear vs exponential scaling comparison
/// Generates items with same base but different scaling to verify scaling factor effect
#[test]
fn test_linear_vs_exponential_scaling_comparison() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    gen.set_quality_data("standard".to_string(), 1);
    gen.set_item_type("gem".to_string(), 1);
    gen.set_item_subtype("gem".to_string(), "emerald".to_string(), 1);

    gen.set_attribute(
        "gem".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "value".to_string(),
            100.0,
            50.0,
            200.0,
            true,
        ),
    );

    gen.set_item(
        "gem".to_string(),
        "emerald".to_string(),
        vec!["emerald".to_string()],
    );

    // Generate with linear scaling
    let options_linear = GeneratorOptions {
        number_of_items: 100,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items_linear = gen.generate_loot(&options_linear, &GeneratorOverrides::empty(), "linear")?;

    // Generate with exponential scaling
    let options_exp = GeneratorOptions {
        number_of_items: 100,
        base_level: 10.0,
        level_variance: 5.0,
        affix_chance: 0.0,
        linear: false,
        scaling_factor: 1.5,
    };

    let items_exp = gen.generate_loot(&options_exp, &GeneratorOverrides::empty(), "exp")?;

    // Calculate average attribute values
    let linear_avg = items_linear
        .iter()
        .map(|i| {
            i.get_attributes()
                .get("value")
                .map(|a| a.get_initial_value())
                .unwrap_or(0.0)
        })
        .sum::<f64>()
        / 100.0;

    let exp_avg = items_exp
        .iter()
        .map(|i| {
            i.get_attributes()
                .get("value")
                .map(|a| a.get_initial_value())
                .unwrap_or(0.0)
        })
        .sum::<f64>()
        / 100.0;

    // Exponential scaling should produce higher average values
    assert!(
        exp_avg > linear_avg,
        "exponential avg {} should be > linear avg {}",
        exp_avg,
        linear_avg
    );

    Ok(())
}

/// Test 7: Override cascade - test all three override types together
/// Verifies overrides work correctly when multiple are specified
#[test]
fn test_override_cascade() -> Result<()> {
    let mut gen = PraedaGenerator::new();

    gen.set_quality_data("common".to_string(), 1);
    gen.set_quality_data("rare".to_string(), 100);

    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_type("armor".to_string(), 100);

    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "axe".to_string(), 100);

    gen.set_item_subtype("armor".to_string(), "chest".to_string(), 1);
    gen.set_item_subtype("armor".to_string(), "legs".to_string(), 100);

    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    gen.set_attribute(
        "armor".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "defense".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    gen.set_item("weapon".to_string(), "sword".to_string(), vec!["sword".to_string()]);
    gen.set_item("weapon".to_string(), "axe".to_string(), vec!["axe".to_string()]);
    gen.set_item("armor".to_string(), "chest".to_string(), vec!["chest".to_string()]);
    gen.set_item("armor".to_string(), "legs".to_string(), vec!["legs".to_string()]);

    // Override all three: force rare sword
    let overrides = GeneratorOverrides {
        quality_override: "rare".to_string(),
        type_override: "weapon".to_string(),
        subtype_override: "sword".to_string(),
    };

    let options = GeneratorOptions {
        number_of_items: 50,
        base_level: 10.0,
        level_variance: 0.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen.generate_loot(&options, &overrides, "overrides")?;

    // All items must be rare swords
    for item in &items {
        assert_eq!(item.get_quality(), "rare");
        assert_eq!(item.get_type(), "weapon");
        assert_eq!(item.get_subtype(), "sword");
        assert_eq!(item.get_name(), "sword");
    }

    Ok(())
}

// ============================================================================
// FILE I/O AND SERIALIZATION TESTS
// ============================================================================


#[test]
fn test_load_toml_data() -> Result<()> {
    let mut gen = PraedaGenerator::new();
    let toml_path = "examples/test_data.toml";

    gen.load_data_toml_from_file(toml_path)?;

    // Verify TOML was loaded
    assert!(gen.get_quality_data().len() > 0);
    assert!(gen.get_item_types().len() > 0);

    Ok(())
}

#[test]
fn test_generate_loot_json() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions {
        number_of_items: 5,
        base_level: 5.0,
        level_variance: 1.0,
        affix_chance: 0.25,
        linear: true,
        scaling_factor: 1.0,
    };

    let json_str = gen.generate_loot_json(&options, &GeneratorOverrides::empty(), "json_gen")?;

    // Verify it's valid JSON and can be parsed
    let _: Vec<Item> = serde_json::from_str(&json_str)?;

    Ok(())
}

// ============================================================================
// MODEL STRUCT TESTS - SETTERS AND MUTATORS
// ============================================================================

#[test]
fn test_item_type_setters() {
    let mut item_type = ItemType::new("weapon".to_string(), HashMap::new(), 1);

    item_type.set_type("armor".to_string());
    assert_eq!(item_type.get_type(), "armor");

    item_type.set_weight(5);
    assert_eq!(item_type.get_weight(), 5);
}

#[test]
fn test_item_attribute_setters() {
    let mut attr = ItemAttribute::new(
        "damage".to_string(),
        10.0,
        1.0,
        20.0,
        false,
    );

    attr.set_name("health".to_string());
    assert_eq!(attr.get_name(), "health");

    attr.set_min(5.0);
    assert_eq!(attr.get_min(), 5.0);

    attr.set_max(50.0);
    assert_eq!(attr.get_max(), 50.0);

    attr.set_required(true);
    assert_eq!(attr.get_required(), true);
}

#[test]
fn test_item_empty() {
    let item = Item::empty();

    assert_eq!(item.get_name(), "");
    assert_eq!(item.get_quality(), "");
    assert_eq!(item.get_type(), "");
    assert_eq!(item.get_subtype(), "");
    assert_eq!(item.get_attributes().len(), 0);
}

#[test]
fn test_item_setters() {
    let mut item = Item::empty();

    item.set_name("sword".to_string());
    assert_eq!(item.get_name(), "sword");

    item.set_quality("rare".to_string());
    assert_eq!(item.get_quality(), "rare");

    item.set_type("weapon".to_string());
    assert_eq!(item.get_type(), "weapon");

    item.set_subtype("one-handed".to_string());
    assert_eq!(item.get_subtype(), "one-handed");
}

#[test]
fn test_item_prefix_suffix_mut() {
    let mut item = Item::empty();

    let prefix = Affix::new("sharp".to_string(), vec![]);
    item.set_prefix(prefix);
    assert_eq!(item.get_prefix().get_name(), "sharp");

    // Test get_prefix_mut
    item.get_prefix_mut().set_name("super_sharp".to_string());
    assert_eq!(item.get_prefix().get_name(), "super_sharp");

    let suffix = Affix::new("of fire".to_string(), vec![]);
    item.set_suffix(suffix);
    assert_eq!(item.get_suffix().get_name(), "of fire");
}

#[test]
fn test_item_attribute_access() {
    let mut item = Item::empty();

    let attr = ItemAttribute::new(
        "damage".to_string(),
        10.0,
        1.0,
        20.0,
        true,
    );

    item.set_attribute("damage".to_string(), attr);

    // Test has_attribute
    assert!(item.has_attribute("damage"));
    assert!(!item.has_attribute("nonexistent"));

    // Test get_attribute
    assert!(item.get_attribute("damage").is_some());
    assert!(item.get_attribute("nonexistent").is_none());

    // Test get_attribute_mut
    if let Some(attr_mut) = item.get_attribute_mut("damage") {
        attr_mut.set_initial_value(15.0);
    }
    assert_eq!(
        item.get_attribute("damage").unwrap().get_initial_value(),
        15.0
    );
}

#[test]
fn test_affix_setters() {
    let mut affix = Affix::empty();

    affix.set_name("fire".to_string());
    assert_eq!(affix.get_name(), "fire");

    let attr = ItemAttribute::new("damage".to_string(), 5.0, 0.0, 10.0, false);
    let attrs = vec![attr];
    affix.set_attributes(attrs);
    assert_eq!(affix.get_attributes().len(), 1);
}

#[test]
fn test_affix_set_attribute() {
    let mut affix = Affix::new("fire".to_string(), vec![]);

    let attr = ItemAttribute::new("damage".to_string(), 5.0, 0.0, 10.0, false);
    affix.set_attribute(attr);
    assert_eq!(affix.get_attributes().len(), 1);

    // Setting same attribute again should replace it
    let attr2 = ItemAttribute::new("damage".to_string(), 10.0, 0.0, 20.0, false);
    affix.set_attribute(attr2);
    assert_eq!(affix.get_attributes().len(), 1);
    assert_eq!(affix.get_attributes()[0].get_initial_value(), 10.0);
}

// ============================================================================
// GENERATOR OPTIONS AND OVERRIDES TESTS
// ============================================================================

#[test]
fn test_generator_options_new() {
    let opts = GeneratorOptions::new(
        10,
        5.0,
        2.0,
        0.5,
        false,
        1.5,
    );

    assert_eq!(opts.number_of_items, 10);
    assert_eq!(opts.base_level, 5.0);
    assert_eq!(opts.level_variance, 2.0);
    assert_eq!(opts.affix_chance, 0.5);
    assert!(!opts.is_linear());
    assert!(opts.is_exponential());
    assert_eq!(opts.scaling_factor, 1.5);
}

#[test]
fn test_generator_options_is_linear() {
    let linear_opts = GeneratorOptions::new(1, 1.0, 1.0, 0.25, true, 1.0);
    assert!(linear_opts.is_linear());
    assert!(!linear_opts.is_exponential());

    let exp_opts = GeneratorOptions::new(1, 1.0, 1.0, 0.25, false, 1.0);
    assert!(!exp_opts.is_linear());
    assert!(exp_opts.is_exponential());
}

#[test]
fn test_generator_overrides_new() {
    let overrides = GeneratorOverrides::new(
        "rare".to_string(),
        "weapon".to_string(),
        "sword".to_string(),
    );

    assert_eq!(overrides.get_quality_override(), "rare");
    assert_eq!(overrides.get_type_override(), "weapon");
    assert_eq!(overrides.get_subtype_override(), "sword");
}

#[test]
fn test_generator_default() {
    let gen = PraedaGenerator::default();
    assert_eq!(gen.get_quality_data().len(), 0);
    assert_eq!(gen.get_item_types().len(), 0);
}

// ============================================================================
// ATTRIBUTE SCALING TESTS
// ============================================================================

#[test]
fn test_generate_value_linear_with_zero_bounds() {
    let mut attr = ItemAttribute::new("damage".to_string(), 10.0, 0.0, 0.0, true);

    // Should set min/max to initial_value when both are 0
    attr.generate_value(5.0, true, 1.0);

    assert_eq!(attr.get_min(), 10.0);
    assert_eq!(attr.get_max(), 10.0);
}

#[test]
fn test_generate_value_exponential_zero_initial() {
    let mut attr = ItemAttribute::new("damage".to_string(), 0.0, 0.0, 0.0, true);

    // Should set initial_value to 1.0 for exponential when 0
    attr.generate_value(5.0, false, 1.5);

    assert_eq!(attr.get_initial_value(), 1.5_f64.powf(5.0));
    assert!(attr.get_initial_value() > 0.0);
}

#[test]
fn test_generate_value_clamps_negative() {
    let mut attr = ItemAttribute::new("damage".to_string(), 5.0, 0.0, 10.0, true);

    // Linear with negative scaling should clamp to 0
    attr.generate_value(10.0, true, -1.0);

    assert_eq!(attr.get_initial_value(), 0.0);
}

#[test]
fn test_attribute_generate_value_exponential() {
    let mut attr = ItemAttribute::new("damage".to_string(), 10.0, 1.0, 100.0, true);

    attr.generate_value(5.0, false, 1.5);

    let expected = 10.0 * (1.5_f64.powf(5.0));
    assert!((attr.get_initial_value() - expected).abs() < 0.01);
}

// ============================================================================
// EDGE CASES AND ERROR HANDLING
// ============================================================================

#[test]
fn test_item_type_has_subtype() {
    let mut item_type = ItemType::new("weapon".to_string(), HashMap::new(), 1);

    // Add a subtype
    item_type.add_subtype("sword".to_string(), 1);

    // Should have the subtype we added
    assert!(item_type.has_subtype("sword"));
    assert!(!item_type.has_subtype("nonexistent"));
}

#[test]
fn test_item_data_struct() {
    let item_data = ItemData::new(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["longsword".to_string(), "shortsword".to_string()],
    );

    assert_eq!(item_data.get_item_type(), "weapon");
    assert_eq!(item_data.get_subtype(), "sword");
    assert_eq!(item_data.get_names().len(), 2);
}

#[test]
fn test_item_data_mutators() {
    let mut item_data = ItemData::new(
        "weapon".to_string(),
        "sword".to_string(),
        vec![],
    );

    item_data.set_item_type("armor".to_string());
    assert_eq!(item_data.get_item_type(), "armor");

    item_data.set_subtype("chest".to_string());
    assert_eq!(item_data.get_subtype(), "chest");

    item_data.add_name("chestplate".to_string());
    assert_eq!(item_data.get_names().len(), 1);
    assert_eq!(item_data.get_names()[0], "chestplate");
}

#[test]
fn test_attribute_updating_same_attribute() {
    let mut gen = PraedaGenerator::new();

    gen.set_quality_data("common".to_string(), 100);
    gen.set_item_type("weapon".to_string(), 1);

    // Set attribute first time
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    // Set same attribute again - should add to initial_value
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            5.0,
            1.0,
            20.0,
            true,
        ),
    );

    assert!(gen.has_attribute("weapon", "", "damage"));
}

#[test]
fn test_get_loot_json() -> Result<()> {
    let mut gen = create_test_generator();

    let options = GeneratorOptions::default();
    gen.generate_loot(&options, &GeneratorOverrides::empty(), "json_test")?;

    let json = gen.get_loot_json("json_test")?;
    assert!(!json.is_empty());

    // Verify it's valid JSON
    let _: Vec<Item> = serde_json::from_str(&json)?;

    Ok(())
}

#[test]
fn test_error_handling_invalid_json() {
    let mut gen = PraedaGenerator::new();
    let invalid_json = "{ invalid json }";

    let result = gen.load_data(invalid_json);
    assert!(result.is_err());
}

#[test]
fn test_weighted_random_select_with_single_item() -> Result<()> {
    let mut gen = PraedaGenerator::new();
    gen.set_quality_data("only_one".to_string(), 1);
    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);
    gen.set_item("weapon".to_string(), "sword".to_string(), vec!["sword".to_string()]);
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    let options = GeneratorOptions::default();
    let items = gen.generate_loot(&options, &GeneratorOverrides::empty(), "single")?;

    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get_quality(), "only_one");

    Ok(())
}

#[test]
fn test_set_item_type_updates_existing() {
    let mut gen = PraedaGenerator::new();

    // Add an item type with weight 1
    gen.set_item_type("weapon".to_string(), 1);
    assert_eq!(gen.get_item_type("weapon").unwrap().get_weight(), 1);

    // Update the same type with weight 5 - tests the rare "type already exists" path
    gen.set_item_type("weapon".to_string(), 5);
    assert_eq!(gen.get_item_type("weapon").unwrap().get_weight(), 5);
}

#[test]
fn test_set_item_subtype_new_type() {
    let mut gen = PraedaGenerator::new();

    // Add subtype to non-existent type - creates new item type with single subtype
    gen.set_item_subtype("armor".to_string(), "chest".to_string(), 2);

    // Verify type was created
    assert!(gen.has_item_type("armor"));
    assert!(gen.has_item_subtype("armor", "chest"));
}

#[test]
fn test_has_item_subtype_nonexistent_type() {
    let mut gen = PraedaGenerator::new();
    gen.set_item_type("weapon".to_string(), 1);

    // Check subtype for non-existent weapon-sword combination - rare path
    assert!(!gen.has_item_subtype("weapon", "nonexistent"));
}

#[test]
fn test_set_initial_value_bounds_from_zero() {
    let mut attr = ItemAttribute::new(
        "test".to_string(),
        50.0,
        0.0,
        0.0,
        true,
    );

    // Both min and max are 0.0, set_initial_value should set them
    assert_eq!(attr.get_min(), 0.0);
    assert_eq!(attr.get_max(), 0.0);

    attr.set_initial_value(25.0);

    // After setting initial value, min/max should be set to initial value
    assert_eq!(attr.get_min(), 25.0);
    assert_eq!(attr.get_max(), 25.0);
    assert_eq!(attr.get_initial_value(), 25.0);
}

#[test]
fn test_has_attribute_missing_attributes() {
    let mut gen = PraedaGenerator::new();

    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);

    // Type and subtype exist, but no attributes set - tests the rare path where attributes aren't found
    assert!(!gen.has_attribute("weapon", "sword", "damage"));
}

#[test]
fn test_get_prefixes_missing() {
    let mut gen = PraedaGenerator::new();

    // No affixes defined - tests the rare path in get_prefixes
    let prefixes = gen.get_prefixes("weapon", "");
    assert_eq!(prefixes.len(), 0);
}

#[test]
fn test_get_suffixes_missing() {
    let mut gen = PraedaGenerator::new();

    // No affixes defined - tests the rare path in get_suffixes
    let suffixes = gen.get_suffixes("weapon", "");
    assert_eq!(suffixes.len(), 0);
}

#[test]
fn test_subtype_metadata_set_and_get() {
    let mut gen = PraedaGenerator::new();

    gen.set_subtype_metadata(
        "weapon".to_string(),
        "one-handed".to_string(),
        "is_two_handed".to_string(),
        serde_json::json!(false),
    );

    let metadata = gen.get_subtype_metadata("weapon", "one-handed", "is_two_handed");
    assert!(metadata.is_some());
    assert_eq!(metadata.unwrap(), &serde_json::json!(false));
}

#[test]
fn test_get_all_subtype_metadata() {
    let mut gen = PraedaGenerator::new();

    gen.set_subtype_metadata(
        "weapon".to_string(),
        "two-handed".to_string(),
        "is_two_handed".to_string(),
        serde_json::json!(true),
    );
    gen.set_subtype_metadata(
        "weapon".to_string(),
        "two-handed".to_string(),
        "weight".to_string(),
        serde_json::json!(15),
    );

    let all_metadata = gen.get_all_subtype_metadata("weapon", "two-handed");
    assert!(all_metadata.is_some());

    let metadata = all_metadata.unwrap();
    assert_eq!(metadata.len(), 2);
    assert_eq!(metadata.get("is_two_handed").unwrap(), &serde_json::json!(true));
    assert_eq!(metadata.get("weight").unwrap(), &serde_json::json!(15));
}

#[test]
fn test_item_metadata_set_and_get() {
    let mut item = Item::new(
        "test_sword".to_string(),
        "common".to_string(),
        "weapon".to_string(),
        "one-handed".to_string(),
        Affix::empty(),
        Affix::empty(),
        HashMap::new(),
    );

    item.set_metadata("is_magical".to_string(), serde_json::json!(true));

    assert!(item.has_metadata("is_magical"));
    assert_eq!(item.get_metadata("is_magical"), Some(&serde_json::json!(true)));
}

#[test]
fn test_item_metadata_get_all() {
    let mut item = Item::new(
        "test_axe".to_string(),
        "rare".to_string(),
        "weapon".to_string(),
        "two-handed".to_string(),
        Affix::empty(),
        Affix::empty(),
        HashMap::new(),
    );

    item.set_metadata("is_two_handed".to_string(), serde_json::json!(true));
    item.set_metadata("weight".to_string(), serde_json::json!(20));

    let all_metadata = item.get_all_metadata();
    assert_eq!(all_metadata.len(), 2);
    assert_eq!(all_metadata.get("is_two_handed").unwrap(), &serde_json::json!(true));
    assert_eq!(all_metadata.get("weight").unwrap(), &serde_json::json!(20));
}

#[test]
fn test_generated_item_contains_subtype_metadata() {
    let mut gen = PraedaGenerator::new();

    // Setup quality data
    gen.set_quality_data("common".to_string(), 100);

    // Setup item type and subtype
    gen.set_item_type("weapon".to_string(), 1);
    gen.set_item_subtype("weapon".to_string(), "sword".to_string(), 1);

    // Set metadata for the subtype
    gen.set_subtype_metadata(
        "weapon".to_string(),
        "sword".to_string(),
        "is_magical".to_string(),
        serde_json::json!(false),
    );

    // Setup attributes
    gen.set_attribute(
        "weapon".to_string(),
        "".to_string(),
        ItemAttribute::new(
            "damage".to_string(),
            10.0,
            1.0,
            20.0,
            true,
        ),
    );

    // Setup item names
    gen.set_item(
        "weapon".to_string(),
        "sword".to_string(),
        vec!["longsword".to_string()],
    );

    // Generate item
    let options = GeneratorOptions {
        number_of_items: 1,
        base_level: 5.0,
        level_variance: 2.0,
        affix_chance: 0.0,
        linear: true,
        scaling_factor: 1.0,
    };

    let items = gen
        .generate_loot(&options, &GeneratorOverrides::empty(), "test")
        .unwrap();

    assert_eq!(items.len(), 1);
    let item = &items[0];

    // Verify the metadata was attached to the generated item
    assert!(item.has_metadata("is_magical"));
    assert_eq!(item.get_metadata("is_magical"), Some(&serde_json::json!(false)));
}

#[test]
fn test_load_metadata_from_toml() {
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
names = ["longsword"]

[[item_affixes]]
item_type = "weapon"
subtype = "sword"
[item_affixes.metadata]
is_legendary = true
rarity_multiplier = 1.5
    "#;

    let mut gen = PraedaGenerator::new();
    gen.load_data_toml(toml_str).unwrap();

    // Verify metadata was loaded
    let metadata = gen.get_subtype_metadata("weapon", "sword", "is_legendary");
    assert!(metadata.is_some());
    assert_eq!(metadata.unwrap(), &serde_json::json!(true));

    let multiplier = gen.get_subtype_metadata("weapon", "sword", "rarity_multiplier");
    assert!(multiplier.is_some());
    assert_eq!(multiplier.unwrap(), &serde_json::json!(1.5));
}

