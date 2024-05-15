#include <ncurses.h>
#include <signal.h>
#include <unistd.h>

#include "snake.h"

static volatile bool gameloop = true;

void sighandler(int sig) { gameloop = false; }

int main() {
    signal(SIGINT, sighandler);
    WINDOW *win = initscr();
    keypad(win, true);
    nodelay(win, true);
    curs_set(0);
    snake_head_pos_t snake_head_pos = {
        .x_pos = FIELD_SIZE_X / 2,
        .y_pos = FIELD_SIZE_Y / 2,
    };
    snake_dir_t snake_dir = {
        .x_dir = -1,
        .y_dir = 0,
    };
    food_pos_t food_pos;
    new_food(&food_pos);

    snake_t *snake = snake_new(&snake_head_pos);
    snake_add_node(snake, &snake_head_pos);

    while (gameloop) {

        int chr = wgetch(win);
        parse_input(chr, &snake_dir);
        set_new_pos(&snake_head_pos, &snake_dir);
        if (!is_valid_position(snake_head_pos.x_pos, snake_head_pos.y_pos)) {
            gameloop = false;
        } else if (snake_is_colliding(snake)) {
            gameloop = false;
        } else if (is_food_pos(&snake_head_pos, &food_pos)) {
            snake_add_node(snake, &snake_head_pos);
            new_food(&food_pos);
        } else {
            snake_propogate(snake, &snake_head_pos);
        }

        werase(win);
        snake_display(snake, win);
        display_food(win, &food_pos);
        display_field(win);
        usleep(100 * 1000);
    }
    snake_remove_last(snake);
    snake_free(snake);

    getch();
    endwin(); /* End curses mode		  */

    printf("SNEK EXITED SUCCESSFULLY\n");

    return 0;
}
