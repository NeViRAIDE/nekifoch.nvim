.PHONY: all clean test

all: clean test

clean:
	rm lua/nekifoch.so

test:
	./test.sh
