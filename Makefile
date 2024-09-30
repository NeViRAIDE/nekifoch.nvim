.PHONY: all clean test install

all: install

clean:
	rm -f lua/nekifoch.so

test: clean
	./test.sh

install:
	chmod +x ./install.sh && ./install.sh
