extends Node

var PORT = 1234
var ADDRESS = "127.0.0.1"
var peer

func _ready() -> void:
	var peer = ENetMultiplayerPeer.new()
	var connection = peer.create_client(ADDRESS, PORT)
	if connection != OK:
		print("Failed to connect.")
		return
	multiplayer.multiplayer_peer = peer
	peer.peer_connected.connect(func(id): print("Succesfully connected."))
	peer.peer_disconnected.connect(func(id): print("Succesfully disconnected."))

func _process(delta) -> void:
	if multiplayer.multiplayer_peer.get_connection_status() == MultiplayerPeer.CONNECTION_CONNECTED:
		var packet = "hello"
		multiplayer.send_bytes(packet.to_utf8_buffer(),1);