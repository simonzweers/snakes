CC=gcc
CFLAGS="-g"
LFLAGS="-lncurses"

all: snake
	echo $^ $@

snake: snake.o
	${CC} ${CFLAGS} -o $@ $^ ${LFLAGS}

snake.o: snake.c
	${CC} ${CFLAGS} -c -o $@ $^

clean:
	rm snake *.o

run: all
	./snake