CC=gcc
CFLAGS="-g"
LFLAGS="-lncurses"

all: snake
	echo $^ $@

snake: snake.o
	${CC} -o $@ $^ ${LFLAGS}

snake.o: snake.c
	${CC} -c -o $@ $^

clean:
	rm snake *.o

run: all
	./snake