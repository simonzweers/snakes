#include <stdio.h>
#include <ncurses.h>
#include <unistd.h>
#include <stdlib.h>

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

typedef struct snake_node
{
	int x;
	int y;
	struct snake_node *next;
} snake_node_t;

typedef struct snake
{
	snake_node_t* head;
} snake_t;

snake_t* snake_new(snake_head_pos_t *snake_head_pos) {
	snake_t* new_snake = (snake_t*) malloc(sizeof(snake_t));
	new_snake->head = (snake_node_t*) malloc(sizeof(snake_node_t));
	new_snake->head->next = NULL;
	new_snake->head->x = snake_head_pos->x_pos;
	new_snake->head->y = snake_head_pos->y_pos;
	printf("Created new snake %p\n", new_snake);
	return new_snake;
}

void snake_add_node(snake_t *snake, snake_head_pos_t *snake_head_pos) {
	snake_node_t *new_node = (snake_node_t *) malloc(sizeof(snake_node_t));
	new_node->x = snake_head_pos->x_pos;
	new_node->y = snake_head_pos->y_pos;
	new_node->next = snake->head;
	snake->head = new_node;
	printf("Adding node: %p | x: %d, y: %d\n", new_node, new_node->x, new_node->y);
}

void snake_remove_last(snake_t *snake) {
	snake_node_t *cursor = snake->head;
	snake_node_t *cursor_previous;
	if (cursor->next == NULL)
	{
		snake->head = NULL;
		free(cursor);
	} else
	{
		while (cursor->next != NULL)
		{
			cursor_previous = cursor;
			cursor = cursor->next;
		}
		cursor_previous->next = NULL;
		printf("Freeing last node: %p | x: %d, y: %d\n", cursor, cursor->x, cursor->y);
		free(cursor);
	}
}

void snake_propogate(snake_t *snake, snake_head_pos_t *snake_head_pos) {
	snake_add_node(snake, snake_head_pos);
	snake_remove_last(snake);
}

void snake_free(snake_t *snake) {
	snake_node_t *cursor = snake->head;
	snake_node_t *cursor_next;
	if (cursor == NULL)
	{
		/* code */
	}
	
	while (cursor != NULL)
	{
		cursor_next = cursor->next;
		printf("Freeing node %p | x: %d, y: %d\n", cursor, cursor->x, cursor->y);
		free(cursor);
		cursor = cursor_next;
	}
	printf("freeing snake %p\n", snake);
	free(snake);
}

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
    // WINDOW* win = initscr();
	// keypad(win, true);
	// nodelay(win, true);
	snake_head_pos_t snake_head_pos = {
		.x_pos = FIELD_SIZE_X / 2,
		.y_pos = FIELD_SIZE_Y / 2,
	};
	snake_dir_t snake_dir = {
		.x_dir = -1,
		.y_dir = 0,
	};
	// while (1) {
	// 	int chr = getch();
	// 	parse_input(chr, &snake_dir);
	// 	set_new_pos(&snake_head_pos, &snake_dir);
	// 	snake_propogate(&snake_head_pos);
	// 	if (!is_valid_position(snake_head_pos.x_pos, snake_head_pos.y_pos))
	// 	{
	// 		break;
	// 	}

	// 	if (is_food_pos())
	// 	{
	// 		snake_add_node();
	// 	} else
	// 	{
	// 		snake_propogate();
	// 	}
		
		
		
	// 	erase();
	// 	mvaddstr(snake_head_pos.y_pos, snake_head_pos.x_pos * 2, "##");
	// 	usleep(100 * 1000);
	// }
	snake_t *snake= snake_new(&snake_head_pos);
	snake_head_pos.x_pos = 1; snake_head_pos.y_pos = 2;
	snake_add_node(snake, &snake_head_pos);
	snake_head_pos.x_pos = 3; snake_head_pos.y_pos = 4;
	snake_add_node(snake, &snake_head_pos);
	snake_head_pos.x_pos = 5; snake_head_pos.y_pos = 6;
	snake_add_node(snake, &snake_head_pos);
	snake_remove_last(snake);
	snake_free(snake);
	
	getch();
	endwin();			/* End curses mode		  */

	return 0;
}
