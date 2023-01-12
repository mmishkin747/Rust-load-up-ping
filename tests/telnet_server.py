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
        
        connections.append(connection)
    except BlockingIOError:
        pass

    for connection in connections:
        try:
            message = connection.recv(4096)
        except BlockingIOError:
            continue

        if message: 
            match message:
                case b'ping 192.168.1.1 repeat 1000 size 1500\r\n' :
                    connection.send(b'Success rate is ... TEST 1 OK\r\n')

    
        


            

