LDFLAGS=-L/usr/local/opt/openssl/lib
CPPFLAGS=-I/usr/local/opt/openssl/include -std=c++11
CC=clang++ $(LDFLAGS) $(CPPFLAGS)

server: server.cpp
	$(CC) $^ -lssl -luWS -lz -luv -o $@

client: client.cpp
	$(CC) $^ -lssl -luWS -lz -luv -o $@
