#include <ncurses.h>
#include <unistd.h>

#include "snake.h"

int main() {
    WINDOW* win = initscr();
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

	snake_t *snake= snake_new(&snake_head_pos);
	snake_add_node(snake, &snake_head_pos);
	snake_add_node(snake, &snake_head_pos);
	snake_add_node(snake, &snake_head_pos);

    bool gameloop = true;
	while (gameloop) {

		int chr = wgetch(win);
		parse_input(chr, &snake_dir);
		set_new_pos(&snake_head_pos, &snake_dir);
		if (!is_valid_position(snake_head_pos.x_pos, snake_head_pos.y_pos))
		{
			gameloop = false;
		} else { // Snake is in valid position
			snake_propogate(snake, &snake_head_pos);
		}

		// 	if (is_food_pos())
		// 	{
		// 		snake_add_node();
		// 	} else
		// 	{
		// 		snake_propogate();
		// 	}
		
		werase(win);
		snake_display(snake, win);
		display_field(win);
		usleep(100 * 1000);
		
	}
	snake_remove_last(snake);
	snake_free(snake);
	
	getch();
	endwin();			/* End curses mode		  */

	return 0;
}
