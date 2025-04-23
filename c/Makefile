CC=gcc
CFLAGS="-g"
LFLAGS="-lncurses"

all: csnake

csnake: main.o snake.o
	${CC} ${CFLAGS} $^ -o $@  ${LFLAGS}

snake.o: snake.c
	${CC} ${CFLAGS} -c -o $@ $^

main.o: main.c
	${CC} ${CFLAGS} -c -o $@ $^

clean:
	rm ./csnake *.o

install:
	cp ./csnake /usr/local/bin/

run: all
	./csnake
