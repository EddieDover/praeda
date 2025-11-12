using System;
using System.Text.Json;
using System.Collections.Generic;
using Praeda;

class Program {
    static int Main() {
        try {
            Console.WriteLine("=== Praeda C# FFI Test ===\n");

            // Create generator
            Console.WriteLine("Creating generator...");
            using var gen = new PraedaGenerator();
            Console.WriteLine("✓ Generator created successfully\n");

            // Test 1: Programmatic Configuration
            Console.WriteLine("--- Test 1: Programmatic Configuration ---");

            Console.WriteLine("Setting qualities...");
            var result = gen.SetQualityData("common", 100);
            if (!result) throw new Exception("Failed to set quality");
            result = gen.SetQualityData("uncommon", 60);
            if (!result) throw new Exception("Failed to set quality");
            result = gen.SetQualityData("rare", 30);
            if (!result) throw new Exception("Failed to set quality");
            Console.WriteLine("✓ Qualities set");

            Console.WriteLine("Setting item types...");
            result = gen.SetItemType("weapon", 2);
            if (!result) throw new Exception("Failed to set item type");
            result = gen.SetItemType("armor", 1);
            if (!result) throw new Exception("Failed to set item type");
            Console.WriteLine("✓ Item types set");

            Console.WriteLine("Setting item subtypes...");
            result = gen.SetItemSubtype("weapon", "sword", 3);
            if (!result) throw new Exception("Failed to set item subtype");
            result = gen.SetItemSubtype("weapon", "axe", 2);
            if (!result) throw new Exception("Failed to set item subtype");
            result = gen.SetItemSubtype("armor", "chest", 1);
            if (!result) throw new Exception("Failed to set item subtype");
            Console.WriteLine("✓ Item subtypes set");

            Console.WriteLine("Setting attributes...");
            result = gen.SetAttribute("weapon", "", "damage", 15.0, 5.0, 30.0, true);
            if (!result) throw new Exception("Failed to set attribute");
            result = gen.SetAttribute("armor", "", "defense", 10.0, 2.0, 20.0, true);
            if (!result) throw new Exception("Failed to set attribute");
            Console.WriteLine("✓ Attributes set");

            Console.WriteLine("Setting item names...");
            result = gen.SetItemNames("weapon", "sword", new[] { "longsword", "shortsword" });
            if (!result) throw new Exception("Failed to set item names");
            result = gen.SetItemNames("weapon", "axe", new[] { "battleaxe" });
            if (!result) throw new Exception("Failed to set item names");
            result = gen.SetItemNames("armor", "chest", new[] { "plate_armor", "leather_armor" });
            if (!result) throw new Exception("Failed to set item names");
            Console.WriteLine("✓ Item names set\n");

            // Test 2: Query Methods
            Console.WriteLine("--- Test 2: Query Methods ---");

            string qualityJson = gen.GetQualityData();
            var qualities = JsonDocument.Parse(qualityJson);
            Console.WriteLine("Quality data: " + qualityJson);
            Console.WriteLine();

            bool hasCommon = gen.HasQuality("common");
            Console.WriteLine("Has quality 'common': " + hasCommon);

            bool hasEpic = gen.HasQuality("epic");
            Console.WriteLine("Has quality 'epic': " + hasEpic);
            Console.WriteLine();

            // Test 3: Generate Loot
            Console.WriteLine("--- Test 3: Loot Generation ---");

            var options = new GenerationOptions {
                NumberOfItems = 5,
                BaseLevel = 15.0,
                LevelVariance = 5.0,
                AffixChance = 0.75,
                Linear = true,
                ScalingFactor = 1.0
            };

            Console.WriteLine("Generating 5 items...");
            string itemsJson = PraedaHelper.GenerateLoot(gen, options);
            var items = JsonDocument.Parse(itemsJson);

            Console.WriteLine("✓ Generated " + items.RootElement.GetArrayLength() + " items:");
            int itemIndex = 1;
            foreach (var item in items.RootElement.EnumerateArray()) {
                string quality = item.GetProperty("quality").GetString();
                string type = item.GetProperty("type").GetString();
                string subtype = item.GetProperty("subtype").GetString();
                string name = item.GetProperty("name").GetString();

                Console.WriteLine($"  {itemIndex}. [{quality}] {type} / {subtype} - {name}");
                itemIndex++;
            }
            Console.WriteLine();

            // Test 4: Generator Info
            Console.WriteLine("--- Test 4: Generator Info ---");
            string infoJson = gen.GetInfo();
            var info = JsonDocument.Parse(infoJson);
            Console.WriteLine("Generator info:");
            Console.WriteLine("  Version: " + info.RootElement.GetProperty("version").GetString());
            Console.WriteLine("  Qualities: " + info.RootElement.GetProperty("qualities").GetInt32());
            Console.WriteLine("  Item types: " + info.RootElement.GetProperty("item_types").GetInt32());
            Console.WriteLine();

            Console.WriteLine("=== All Tests Passed! ===");
            return 0;

        } catch (Exception ex) {
            Console.Error.WriteLine("Error: " + ex.Message);
            Console.Error.WriteLine(ex.StackTrace);
            return 1;
        }
    }
}
