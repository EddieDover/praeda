extends Node3D

var generator: PraedaGodotGenerator
var loot_container: Node3D
var tooltip_label: RichTextLabel
var tooltip_panel: PanelContainer

func _ready():
	print("=== Praeda Godot 3D Visualizer ===")
	
	setup_environment()
	
	setup_ui()
	
	setup_generator()

func setup_environment():
	# Camera
	var camera = Camera3D.new()
	add_child(camera)
	camera.position = Vector3(0, 4, 8)
	camera.look_at(Vector3.ZERO)
	
	# Light
	var light = DirectionalLight3D.new()
	add_child(light)
	light.position = Vector3(5, 10, 5)
	light.look_at(Vector3.ZERO)
	light.shadow_enabled = true
	
	# Floor (optional, for reference)
	var floor_mesh = MeshInstance3D.new()
	var plane = PlaneMesh.new()
	plane.size = Vector2(20, 20)
	floor_mesh.mesh = plane
	var floor_mat = StandardMaterial3D.new()
	floor_mat.albedo_color = Color(0.2, 0.2, 0.2)
	floor_mesh.material_override = floor_mat
	add_child(floor_mesh)
	
	# Container for loot cubes
	loot_container = Node3D.new()
	add_child(loot_container)

func setup_ui():
	var canvas = CanvasLayer.new()
	add_child(canvas)
	
	# Generate Button
	var btn = Button.new()
	btn.text = "Generate Loot"
	btn.position = Vector2(20, 20)
	btn.size = Vector2(200, 50)
	btn.pressed.connect(_on_generate_pressed)
	canvas.add_child(btn)
	
	# Tooltip Panel
	tooltip_panel = PanelContainer.new()
	tooltip_panel.visible = false
	canvas.add_child(tooltip_panel)
	
	tooltip_label = RichTextLabel.new()
	tooltip_label.bbcode_enabled = true
	tooltip_label.fit_content = true
	tooltip_label.autowrap_mode = TextServer.AUTOWRAP_OFF
	tooltip_label.custom_minimum_size = Vector2(200, 0)
	tooltip_panel.add_child(tooltip_label)

func setup_generator():
	generator = PraedaGodotGenerator.new()
	
	# Configure Qualities (Rarity Tiers)
	generator.set_quality_data("Poor", 150)
	generator.set_quality_data("Common", 100)
	generator.set_quality_data("Uncommon", 60)
	generator.set_quality_data("Rare", 30)
	generator.set_quality_data("Epic", 10)
	generator.set_quality_data("Legendary", 5)
	generator.set_quality_data("Artifact", 2)
	generator.set_quality_data("Heirloom", 1)
	
	# Configure Types & Subtypes
	# Weapons
	generator.set_item_type("Weapon", 10)
	generator.set_item_subtype("Weapon", "Sword", 5)
	generator.set_item_subtype("Weapon", "Axe", 5)
	generator.set_item_subtype("Weapon", "Mace", 4)
	generator.set_item_subtype("Weapon", "Staff", 3)
	generator.set_item_subtype("Weapon", "Bow", 4)
	
	# Armor
	generator.set_item_type("Armor", 10)
	generator.set_item_subtype("Armor", "Head", 5)
	generator.set_item_subtype("Armor", "Chest", 5)
	generator.set_item_subtype("Armor", "Legs", 5)
	generator.set_item_subtype("Armor", "Boots", 5)
	
	# Accessories
	generator.set_item_type("Accessory", 5)
	generator.set_item_subtype("Accessory", "Ring", 5)
	generator.set_item_subtype("Accessory", "Amulet", 3)
	
	# Configure Attributes
	# Global / Common
	generator.set_attribute("", "", "Value", 10.0, 1.0, 1000.0, true)
	
	# Weapon Specific
	generator.set_attribute("Weapon", "", "Damage", 15.0, 5.0, 100.0, true)
	generator.set_attribute("Weapon", "", "AttackSpeed", 1.2, 0.5, 2.5, true)
	generator.set_attribute("Weapon", "", "CritChance", 5.0, 0.0, 50.0, false)
	generator.set_attribute("Weapon", "", "Durability", 100.0, 10.0, 100.0, true)
	
	# Armor Specific
	generator.set_attribute("Armor", "", "Defense", 20.0, 5.0, 200.0, true)
	generator.set_attribute("Armor", "", "HealthBonus", 50.0, 10.0, 500.0, false)
	generator.set_attribute("Armor", "", "FireResist", 10.0, 0.0, 75.0, false)
	
	# Accessory Specific
	generator.set_attribute("Accessory", "", "Mana", 20.0, 10.0, 200.0, true)
	generator.set_attribute("Accessory", "", "Intelligence", 5.0, 1.0, 50.0, false)
	generator.set_attribute("Accessory", "", "Strength", 5.0, 1.0, 50.0, false)
	
	# Configure Names
	# Weapons
	generator.set_item_names("Weapon", "Sword", ["Longsword", "Shortsword", "Claymore", "Katana", "Broadsword", "Rapier"])
	generator.set_item_names("Weapon", "Axe", ["Battleaxe", "Hatchet", "Greataxe", "Tomahawk", "Double Axe"])
	generator.set_item_names("Weapon", "Mace", ["Morningstar", "Warhammer", "Club", "Flail", "Maul"])
	generator.set_item_names("Weapon", "Staff", ["Wooden Staff", "Quarterstaff", "Arcane Staff", "Elder Staff"])
	generator.set_item_names("Weapon", "Bow", ["Shortbow", "Longbow", "Recurve Bow", "Crossbow"])
	
	# Armor
	generator.set_item_names("Armor", "Head", ["Helmet", "Coif", "Hood", "Crown", "Cap"])
	generator.set_item_names("Armor", "Chest", ["Breastplate", "Tunic", "Robe", "Chainmail", "Plate Mail"])
	generator.set_item_names("Armor", "Legs", ["Greaves", "Leggings", "Pants", "Kilt"])
	generator.set_item_names("Armor", "Boots", ["Boots", "Sandals", "Sabatons", "Shoes"])
	
	# Accessories
	generator.set_item_names("Accessory", "Ring", ["Gold Ring", "Silver Ring", "Signet Ring", "Band"])
	generator.set_item_names("Accessory", "Amulet", ["Necklace", "Pendant", "Choker", "Talisman"])
	
	print("Generator configured with extended data.")

func _on_generate_pressed():
	# Clear existing loot
	for child in loot_container.get_children():
		child.queue_free()
	
	var options = {
		"number_of_items": 10,
		"base_level": 25.0,
		"level_variance": 5.0,
		"affix_chance": 0.6,
		"scaling_factor": 1.2
	}
	
	print("Generating items...")
	var items = generator.generate_loot(options)
	spawn_items(items)

func spawn_items(items):
	var count = items.size()
	var spacing = 1.2
	var items_per_row = 5
	var start_x = -((items_per_row - 1) * spacing) / 2.0
	
	for i in range(count):
		var item = items[i]
		var row = i / items_per_row
		var col = i % items_per_row
		
		var x = start_x + (col * spacing)
		var z = row * spacing
		
		var pos = Vector3(x, 1.0, z)
		spawn_cube(item, pos)

func spawn_cube(item, pos):
	var body = StaticBody3D.new()
	body.position = pos
	loot_container.add_child(body)
	
	# Mesh
	var mesh_inst = MeshInstance3D.new()
	var mesh = BoxMesh.new()
	mesh.size = Vector3(0.8, 0.8, 0.8)
	
	var material = StandardMaterial3D.new()
	material.albedo_color = get_color_for_quality(item.quality)
	material.emission_enabled = true
	material.emission = get_color_for_quality(item.quality)
	material.emission_energy_multiplier = 0.2
	mesh.material = material
	
	mesh_inst.mesh = mesh
	body.add_child(mesh_inst)
	
	# Collision
	var shape = CollisionShape3D.new()
	var box = BoxShape3D.new()
	box.size = Vector3(0.8, 0.8, 0.8)
	shape.shape = box
	body.add_child(shape)
	
	# Mouse Interaction
	body.mouse_entered.connect(func(): show_tooltip(item))
	body.mouse_exited.connect(func(): hide_tooltip())
	
	# Animation (randomized bobbing)
	var time_offset = randf() * 2.0
	
	# Use a timer for initial offset so it doesn't repeat every loop
	get_tree().create_timer(time_offset).timeout.connect(func():
		if not is_instance_valid(mesh_inst): return
		
		# Create tween bound to the mesh instance so it stops when mesh is freed
		var tween = mesh_inst.create_tween().set_loops()
		tween.tween_property(mesh_inst, "position:y", 0.2, 1.0).as_relative().set_trans(Tween.TRANS_SINE)
		tween.tween_property(mesh_inst, "position:y", -0.2, 1.0).as_relative().set_trans(Tween.TRANS_SINE)
	)
	
	# Rotation animation
	var rot_tween = mesh_inst.create_tween().set_loops()
	rot_tween.tween_property(mesh_inst, "rotation:y", deg_to_rad(360), 4.0 + randf()).as_relative()

func get_color_for_quality(quality):
	match quality:
		"Poor": return Color.DIM_GRAY
		"Common": return Color.WHITE
		"Uncommon": return Color.GREEN
		"Rare": return Color.BLUE
		"Epic": return Color.PURPLE
		"Legendary": return Color.ORANGE
		"Artifact": return Color.RED
		"Heirloom": return Color.GOLD
		_: return Color.WHITE

func show_tooltip(item):
	var text = "[b]%s[/b]\n%s %s\nQuality: [color=#%s]%s[/color]\n\n[u]Attributes:[/u]" % [
		item.name, 
		item.quality, 
		item.subtype, 
		get_color_for_quality(item.quality).to_html(),
		item.quality
	]
	
	for key in item.attributes:
		text += "\n%s: %.1f" % [key.capitalize(), item.attributes[key]]
	
	tooltip_label.text = text
	tooltip_panel.visible = true
	
	# Force update to get correct size
	tooltip_panel.reset_size()

func hide_tooltip():
	tooltip_panel.visible = false

func _process(_delta):
	if tooltip_panel.visible:
		var mouse_pos = get_viewport().get_mouse_position()
		var panel_size = tooltip_panel.size
		# Position above cursor, centered horizontally
		tooltip_panel.position = mouse_pos + Vector2(-panel_size.x / 2, -panel_size.y - 20)
