CC=gcc
CFLAGS="-g"
LFLAGS="-lncurses"

all: snake

snake: main.o snake.o
	${CC} ${CFLAGS} $^ -o $@  ${LFLAGS}

snake.o: snake.c
	${CC} ${CFLAGS} -c -o $@ $^

main.o: main.c
	${CC} ${CFLAGS} -c -o $@ $^

clean:
	rm snake *.o

run: all
	./snake