#include "inkless.h"

/* --- Dispatcher --- */

void command_dispatch(AppState *app, int key) {
    if (app->search_failed) app->search_failed = false;
    if (app->search_wrapped) app->search_wrapped = false;

    if (app->show_help) {
        if (key == KEY_HELP || key == KEY_ESC) app->show_help = false;
        else if (key == KEY_QUIT) cmd_sys_quit(app);
        return;
    }

    switch (key) {
        case KEY_UP: case 'k':    cmd_nav_up(app); break;
        case KEY_DOWN: case 'j':  cmd_nav_down(app); break;
        case KEY_PAGE_UP: case 'b': cmd_nav_page_up(app); break;
        case KEY_PAGE_DOWN: case ' ': case 'f': cmd_nav_page_down(app); break;
        case KEY_HALF_UP: case 'u': cmd_nav_half_up(app); break;
        case KEY_HALF_DOWN: case 'd': cmd_nav_half_down(app); break;
        case KEY_HOME: case 'g': case '<': cmd_nav_home(app); break;
        case KEY_END: case 'G': case '>': cmd_nav_end(app); break;
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

/* --- Navigation --- */

static void clamp_scroll(AppState *app) {
    int view_h = app->ts.rows - 1;
    if (app->scroll_y + view_h > (int)app->layout.count)
        app->scroll_y = (int)app->layout.count - view_h;
    if (app->scroll_y < 0) app->scroll_y = 0;
}

void cmd_nav_up(AppState *app) { if (app->scroll_y > 0) app->scroll_y--; }
void cmd_nav_down(AppState *app) { app->scroll_y++; clamp_scroll(app); }
void cmd_nav_page_up(AppState *app) { app->scroll_y -= (app->ts.rows - 1); clamp_scroll(app); }
void cmd_nav_page_down(AppState *app) { app->scroll_y += (app->ts.rows - 1); clamp_scroll(app); }
void cmd_nav_half_up(AppState *app) { app->scroll_y -= (app->ts.rows - 1) / 2; clamp_scroll(app); }
void cmd_nav_half_down(AppState *app) { app->scroll_y += (app->ts.rows - 1) / 2; clamp_scroll(app); }
void cmd_nav_home(AppState *app) { app->scroll_y = 0; }
void cmd_nav_end(AppState *app) { app->scroll_y = (int)app->layout.count; clamp_scroll(app); }

/* --- Search --- */

void cmd_search_forward(AppState *app) {
    char pattern[256];
    view_read_prompt(app, '/', pattern, sizeof(pattern));
    if (pattern[0]) {
        strncpy(app->last_pattern, pattern, sizeof(app->last_pattern)-1);
        app->last_search_dir = 1;
        utils_do_search(app, app->last_pattern, 1);
    }
}

void cmd_search_backward(AppState *app) {
    char pattern[256];
    view_read_prompt(app, '?', pattern, sizeof(pattern));
    if (pattern[0]) {
        strncpy(app->last_pattern, pattern, sizeof(app->last_pattern)-1);
        app->last_search_dir = -1;
        utils_do_search(app, app->last_pattern, -1);
    }
}

void cmd_search_next(AppState *app) {
    if (app->last_pattern[0]) utils_do_search(app, app->last_pattern, app->last_search_dir);
}

void cmd_search_prev(AppState *app) {
    if (app->last_pattern[0]) utils_do_search(app, app->last_pattern, -app->last_search_dir);
}

/* --- System --- */

void cmd_sys_help(AppState *app) { app->show_help = !app->show_help; }
void cmd_sys_quit(AppState *app) { app->running = false; }

void cmd_sys_colon(AppState *app) {
    char buf[256];
    view_read_prompt(app, ':', buf, sizeof(buf));
    if (buf[0] == '\0') return;

    if (strcmp(buf, "n") == 0 && app->current_file_index < app->num_files - 1)
        app_switch_file(app, app->current_file_index + 1);
    else if (strcmp(buf, "p") == 0 && app->current_file_index > 0)
        app_switch_file(app, app->current_file_index - 1);
    else if (strcmp(buf, "q") == 0) cmd_sys_quit(app);
    else if (strcmp(buf, "i") == 0) app->search_case_insensitive = !app->search_case_insensitive;
    else if (strcmp(buf, "N") == 0) {
        app->show_line_numbers = !app->show_line_numbers;
    } else {
        char *end;
        long line = strtol(buf, &end, 10);
        if (*end == '\0' && line > 0) {
            if (line > (long)app->doc.line_count) line = (long)app->doc.line_count;
            app->scroll_y = (int)app->layout.raw_to_display[line - 1];
            clamp_scroll(app);
        }
    }
}
