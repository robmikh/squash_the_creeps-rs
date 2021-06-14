extends Node

export(PackedScene) var mob_scene


func _ready():
	randomize()

func on_mob_timer_timeout():
	# Create a Mob instance and add it to the scene.
	var mob = mob_scene.instance()
	var spawner_location = $Spawner.transform.origin
	var player_position = $ShootAt.transform.origin

	add_child(mob)
	mob.initialize(spawner_location, player_position)

