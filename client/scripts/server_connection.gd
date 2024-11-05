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
	multiplayer.multiplayer_peer.poll()
	if multiplayer.multiplayer_peer.get_connection_status() == MultiplayerPeer.CONNECTION_CONNECTED:
		var packet_type = 01 # Hello Packet
		var variant_type = 01
		var message = "hello"
		var packet: PackedByteArray = PackedByteArray()
		packet.append(packet_type)
		packet.append(variant_type)
		packet.append_array(message.to_utf8_buffer())
		multiplayer.multiplayer_peer.put_packet(packet)
	if multiplayer.multiplayer_peer.get_connection_status() == MultiplayerPeer.CONNECTION_DISCONNECTED:
		get_tree().change_scene_to_file("res://scenes/disconnect_popup.tscn")
