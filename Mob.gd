extends KinematicBody

# Minimum speed of the mob in meters per second.
export var min_speed = 10
# Maximum speed of the mob in meters per second.
export var max_speed = 18

var velocity = Vector3.ZERO


func _physics_process(_delta):
	move_and_slide(velocity)


func initialize(start_position, player_position):
	translation = start_position
	look_at(player_position, Vector3.UP)

	velocity = Vector3.FORWARD * max_speed
	velocity = velocity.rotated(Vector3.UP, rotation.y)


func on_visibility_screen_exited():
	queue_free()
