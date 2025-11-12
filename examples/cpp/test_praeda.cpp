#include "praeda.hpp"
#include <nlohmann/json.hpp>
#include <iostream>

using json = nlohmann::json;

int main() {
    try {
        std::cout << "=== Praeda C++ FFI Test ===" << std::endl << std::endl;

        // Create generator
        std::cout << "Creating generator..." << std::endl;
        auto gen = praeda::Generator::create();
        std::cout << "✓ Generator created successfully" << std::endl << std::endl;

        // Test 1: Programmatic Configuration
        std::cout << "--- Test 1: Programmatic Configuration ---" << std::endl;

        std::cout << "Setting qualities..." << std::endl;
        gen->set_quality_data("common", 100);
        gen->set_quality_data("uncommon", 60);
        gen->set_quality_data("rare", 30);
        std::cout << "✓ Qualities set" << std::endl;

        std::cout << "Setting item types..." << std::endl;
        gen->set_item_type("weapon", 2);
        gen->set_item_type("armor", 1);
        std::cout << "✓ Item types set" << std::endl;

        std::cout << "Setting item subtypes..." << std::endl;
        gen->set_item_subtype("weapon", "sword", 3);
        gen->set_item_subtype("weapon", "axe", 2);
        gen->set_item_subtype("armor", "chest", 1);
        std::cout << "✓ Item subtypes set" << std::endl;

        std::cout << "Setting attributes..." << std::endl;
        gen->set_attribute("weapon", "",
            praeda::ItemAttribute("damage", 15.0, 5.0, 30.0, true));
        gen->set_attribute("armor", "",
            praeda::ItemAttribute("defense", 10.0, 2.0, 20.0, true));
        std::cout << "✓ Attributes set" << std::endl;

        std::cout << "Setting item names..." << std::endl;
        gen->set_item_names("weapon", "sword", {"longsword", "shortsword"});
        gen->set_item_names("weapon", "axe", {"battleaxe"});
        gen->set_item_names("armor", "chest", {"plate_armor", "leather_armor"});
        std::cout << "✓ Item names set" << std::endl << std::endl;

        // Test 2: Query Methods
        std::cout << "--- Test 2: Query Methods ---" << std::endl;

        std::string quality_json = gen->get_quality_data();
        json qualities = json::parse(quality_json);
        std::cout << "Quality data: " << qualities.dump(2) << std::endl << std::endl;

        bool has_common = gen->has_quality("common");
        std::cout << "Has quality 'common': " << (has_common ? "true" : "false") << std::endl;

        bool has_epic = gen->has_quality("epic");
        std::cout << "Has quality 'epic': " << (has_epic ? "true" : "false") << std::endl << std::endl;

        // Test 3: Generate Loot
        std::cout << "--- Test 3: Loot Generation ---" << std::endl;

        json options = {
            {"number_of_items", 5},
            {"base_level", 15.0},
            {"level_variance", 5.0},
            {"affix_chance", 0.75},
            {"linear", true},
            {"scaling_factor", 1.0}
        };

        std::cout << "Generating 5 items..." << std::endl;
        std::string items_json = gen->generate_loot(options.dump());
        json items = json::parse(items_json);

        std::cout << "✓ Generated " << items.size() << " items:" << std::endl;
        for (size_t i = 0; i < items.size(); ++i) {
            const auto& item = items[i];
            std::cout << "  " << (i+1) << ". [" << item["quality"].get<std::string>()
                      << "] " << item["type"].get<std::string>()
                      << " / " << item["subtype"].get<std::string>()
                      << " - " << item["name"].get<std::string>() << std::endl;
        }
        std::cout << std::endl;

        // Test 4: Generator Info
        std::cout << "--- Test 4: Generator Info ---" << std::endl;
        std::string info_json = gen->info();
        json info = json::parse(info_json);
        std::cout << "Generator info:" << std::endl;
        std::cout << "  Version: " << info["version"].get<std::string>() << std::endl;
        std::cout << "  Qualities: " << info["qualities"].get<int>() << std::endl;
        std::cout << "  Item types: " << info["item_types"].get<int>() << std::endl;
        std::cout << std::endl;

        std::cout << "=== All Tests Passed! ===" << std::endl;
        return 0;

    } catch (const praeda::Exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    } catch (const std::exception& e) {
        std::cerr << "Unexpected error: " << e.what() << std::endl;
        return 1;
    }
}
