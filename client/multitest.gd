extends Node

var PORT = 1234
var ADDRESS = "127.0.0.1"

func _ready() -> void:
	var peer = ENetMultiplayerPeer.new()
	var connection = peer.create_client(ADDRESS, PORT)
	if connection != OK:
		print("Failed to connect.")
		return
	multiplayer.multiplayer_peer = peer
	peer.peer_connected.connect(func(id): print("Succesfully connected."))
	peer.peer_disconnected.connect(func(id): print("Succesfully disconnected."))
