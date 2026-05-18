#include "ink.h"

/* --- Dispatcher --- */

void command_dispatch(AppState *app, int key) {
    if (app->search_failed) {
        app->search_failed = false;
    }

    if (app->show_help) {
        if (key == KEY_HELP || key == KEY_ESC) {
            app->show_help = false;
        } else if (key == KEY_QUIT) {
            cmd_sys_quit(app);
        }
        return;
    }

    switch (key) {
        case KEY_UP:              cmd_nav_up(app); break;
        case KEY_DOWN:            cmd_nav_down(app); break;
        case KEY_PAGE_UP:         cmd_nav_page_up(app); break;
        case KEY_PAGE_DOWN:       cmd_nav_page_down(app); break;
        case KEY_HALF_UP:         cmd_nav_half_up(app); break;
        case KEY_HALF_DOWN:       cmd_nav_half_down(app); break;
        case KEY_HOME:            cmd_nav_home(app); break;
        case KEY_END:             cmd_nav_end(app); break;
        case KEY_SEARCH_FORWARD:  cmd_search_forward(app); break;
        case KEY_SEARCH_BACKWARD: cmd_search_backward(app); break;
        case KEY_SEARCH_NEXT:     cmd_search_next(app); break;
        case KEY_SEARCH_PREV:     cmd_search_prev(app); break;
        case ':':                 cmd_sys_colon(app); break;
        case KEY_HELP:            cmd_sys_help(app); break;
        case KEY_ESC:             app->last_pattern[0] = '\0'; break;
        case KEY_QUIT:            cmd_sys_quit(app); break;
    }
}

/* --- Navigation Commands --- */

void cmd_nav_up(AppState *app) {
    if (app->scroll_y > 0) app->scroll_y--;
}

void cmd_nav_down(AppState *app) {
    int view_height = app->ts.rows - 1;
    if (app->scroll_y + view_height < (int)app->layout.count) {
        app->scroll_y++;
    }
}

void cmd_nav_page_up(AppState *app) {
    int view_height = app->ts.rows - 1;
    app->scroll_y -= view_height;
    if (app->scroll_y < 0) app->scroll_y = 0;
}

void cmd_nav_page_down(AppState *app) {
    int view_height = app->ts.rows - 1;
    app->scroll_y += view_height;
    if (app->scroll_y + view_height > (int)app->layout.count) {
        app->scroll_y = (int)app->layout.count - view_height;
        if (app->scroll_y < 0) app->scroll_y = 0;
    }
}

void cmd_nav_half_up(AppState *app) {
    int view_height = app->ts.rows - 1;
    app->scroll_y -= view_height / 2;
    if (app->scroll_y < 0) app->scroll_y = 0;
}

void cmd_nav_half_down(AppState *app) {
    int view_height = app->ts.rows - 1;
    app->scroll_y += view_height / 2;
    if (app->scroll_y + view_height > (int)app->layout.count) {
        app->scroll_y = (int)app->layout.count - view_height;
        if (app->scroll_y < 0) app->scroll_y = 0;
    }
}

void cmd_nav_home(AppState *app) {
    app->scroll_y = 0;
}

void cmd_nav_end(AppState *app) {
    int view_height = app->ts.rows - 1;
    app->scroll_y = (int)app->layout.count - view_height;
    if (app->scroll_y < 0) app->scroll_y = 0;
}

/* --- Search Commands --- */

void cmd_search_forward(AppState *app) {
    char pattern[256];
    view_read_prompt(app, '/', pattern, sizeof(pattern));
    if (pattern[0] != '\0') {
        strncpy(app->last_pattern, pattern, sizeof(app->last_pattern) - 1);
        app->last_search_dir = 1;
        utils_do_search(app, app->last_pattern, 1);
    }
}

void cmd_search_backward(AppState *app) {
    char pattern[256];
    view_read_prompt(app, '?', pattern, sizeof(pattern));
    if (pattern[0] != '\0') {
        strncpy(app->last_pattern, pattern, sizeof(app->last_pattern) - 1);
        app->last_search_dir = -1;
        utils_do_search(app, app->last_pattern, -1);
    }
}

void cmd_search_next(AppState *app) {
    if (app->last_pattern[0] != '\0') {
        utils_do_search(app, app->last_pattern, app->last_search_dir);
    }
}

void cmd_search_prev(AppState *app) {
    if (app->last_pattern[0] != '\0') {
        utils_do_search(app, app->last_pattern, -app->last_search_dir);
    }
}

/* --- System Commands --- */

void cmd_sys_help(AppState *app) {
    app->show_help = !app->show_help;
}

void cmd_sys_quit(AppState *app) {
    app->running = false;
}

void cmd_sys_colon(AppState *app) {
    // Briefly show the colon prompt at the bottom
    RenderBuf rb;
    rb_init(&rb);
    rb_printf(&rb, "\x1b[%d;1H\x1b[2K:", app->ts.rows);
    rb_flush(&rb);
    rb_free(&rb);

    terminal_show_cursor();
    int key = input_read_key();
    terminal_hide_cursor();

    if (key == 'n') {
        if (app->current_file_index < app->num_files - 1) {
            app_switch_file(app, app->current_file_index + 1);
        }
    } else if (key == 'p') {
        if (app->current_file_index > 0) {
            app_switch_file(app, app->current_file_index - 1);
        }
    } else if (key == 'q') {
        cmd_sys_quit(app);
    }
}
