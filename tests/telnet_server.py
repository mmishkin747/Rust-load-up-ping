import sys
import socket


server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.setblocking(False)
server.bind(('localhost', int(sys.argv[1])))
server.listen(5)

connections = []

def auth(connections):
    connection.send(b"username:")
    while True:
        try:
            message = connection.recv(4096)
        except BlockingIOError:
            continue
        if message:
            print(message)
        match message:
                case b'admin\r\n':
                    connection.send(b'password:')
                case b'1111\r\n':
                    connection.send(b'auth successful\r\n')
                    return connection

while True:
    try:
        connection, address = server.accept()
        connection.setblocking(False)
        connection.send(b"Hello! I'm test stend\r\n")
        auth(connection)
        connections.append(connection)
    except BlockingIOError:
        pass

        


            

