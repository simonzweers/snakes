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
	rm snake *.o

install:
	cp ./snake /usr/local/bin/

run: all
	./snake