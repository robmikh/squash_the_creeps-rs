extends Node

export(PackedScene) var mob_scene


func _ready():
	randomize()

func on_mob_timer_timeout():
	# Create a Mob instance and add it to the scene.
	var mob = mob_scene.instance()

	# Choose a random location on Path2D.
	var mob_spawn_location = get_node("SpawnPath/SpawnLocation")
	mob_spawn_location.unit_offset = 0.5 #randf()

	#var player_position = $Player.transform.origin
	var player_position = Vector3(-4.7574863, 0.8057514, -6.1575346)

	add_child(mob)
	mob.initialize(mob_spawn_location.translation, player_position)

