#include <stdio.h>
#include <ncurses.h>
#include <unistd.h>

const int FIELD_SIZE_X = 30;
const int FIELD_SIZE_Y = 20;

typedef struct snake_dir
{
	int x_dir;
	int y_dir;
} snake_dir_t;

typedef struct snake_head_pos
{
	int x_pos;
	int y_pos;
} snake_head_pos_t;


void parse_input(int input_char, snake_dir_t* snake_dir_ptr) {
	// Y coordinate is inverted because of terminal coordinates
	switch (input_char)
	{
	case KEY_UP:
		snake_dir_ptr->x_dir = 0;
		snake_dir_ptr->y_dir = -1;
		break;
	case KEY_DOWN:
		snake_dir_ptr->x_dir = 0;
		snake_dir_ptr->y_dir = 1;
		break;
	case KEY_LEFT:
		snake_dir_ptr->x_dir = -1;
		snake_dir_ptr->y_dir = 0;
		break;
	case KEY_RIGHT:
		snake_dir_ptr->x_dir = 1;
		snake_dir_ptr->y_dir = 0;
		break;
	default:
		break;
	}
}

void set_new_pos(snake_head_pos_t* snake_head_pos, snake_dir_t *snake_dir) {
	snake_head_pos->x_pos += snake_dir->x_dir;
	snake_head_pos->y_pos += snake_dir->y_dir;
}
bool is_valid_position(int x, int y) {
	if ((x >= 0 && x < FIELD_SIZE_X) && (y >= 0 && y < FIELD_SIZE_Y))
	{
		return true;
	} else
	{
		return false;
	}
}

int main() {
    WINDOW* win = initscr();
	keypad(win, true);
	nodelay(win, true);
	snake_head_pos_t snake_head_pos = {
		.x_pos = FIELD_SIZE_X / 2,
		.y_pos = FIELD_SIZE_Y / 2,
	};
	snake_dir_t snake_dir = {
		.x_dir = -1,
		.y_dir = 0,
	};
	while (1) {
		int chr = getch();
		parse_input(chr, &snake_dir);
		set_new_pos(&snake_head_pos, &snake_dir);
		if (!is_valid_position(snake_head_pos.x_pos, snake_head_pos.y_pos))
		{
			break;
		}
		
		erase();
		mvaddstr(snake_head_pos.y_pos, snake_head_pos.x_pos * 2, "##");
		usleep(100 * 1000);
	}
	
	endwin();			/* End curses mode		  */

	return 0;
}
