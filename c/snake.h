#ifndef SNAKE_H
#define SNAKE_H

#include <ncurses.h>

static const int FIELD_SIZE_X = 30;
static const int FIELD_SIZE_Y = 20;

typedef struct snake_dir {
    int x_dir;
    int y_dir;
} snake_dir_t;

typedef struct snake_head_pos {
    int x_pos;
    int y_pos;
} snake_head_pos_t;

typedef struct snake_node {
    int x;
    int y;
    struct snake_node *next;
} snake_node_t;

typedef struct snake {
    int length;
    snake_node_t *head;
} snake_t;

typedef struct food_pos {
    int x;
    int y;
} food_pos_t;

snake_t *snake_new(snake_head_pos_t *snake_head_pos);

void snake_add_node(snake_t *snake, snake_head_pos_t *snake_head_pos);

void snake_remove_last(snake_t *snake);

void snake_propogate(snake_t *snake, snake_head_pos_t *snake_head_pos);

void snake_display(snake_t *snake, WINDOW *win);

bool snake_is_colliding(snake_t *snake);

void snake_free(snake_t *snake);

void display_field(WINDOW *win);

void parse_input(int input_char, snake_dir_t *snake_dir_ptr);

void set_new_pos(snake_head_pos_t *snake_head_pos, snake_dir_t *snake_dir);

bool is_valid_position(int x, int y);

bool is_food_pos(snake_head_pos_t *snake_head_pos, food_pos_t *food_pos);

void new_food(food_pos_t *food_pos);

void display_food(WINDOW *win, food_pos_t *food_pos);

#endif // __SNAKE_H__
