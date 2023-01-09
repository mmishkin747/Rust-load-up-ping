import sys
import socket


server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.setblocking(False)
server.bind(('localhost', int(sys.argv[1])))
server.listen(5)

connections = []

while True:
    try:
        connection, address = server.accept()
        connection.setblocking(False)
        connection.send(b"username:")
        connections.append(connection)
    except BlockingIOError:
        pass

    for connection in connections:
        try:
            message = connection.recv(4096)
        except BlockingIOError:
            continue

        for connection in connections:
            connection.send(message)