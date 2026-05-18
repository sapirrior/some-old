#ifndef INKLESS_H
#define INKLESS_H

#if defined(__APPLE__) || defined(__FreeBSD__) || defined(__OpenBSD__) || defined(__NetBSD__)
#define _DEFAULT_SOURCE
#define _DARWIN_C_SOURCE
#endif

#ifndef _POSIX_C_SOURCE
#define _POSIX_C_SOURCE 200809L
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>
#include <stdbool.h>
#include <stdarg.h>
#include <signal.h>
#include <termios.h>

/* --- CONSTANTS --- */

#define INKLESS_VERSION "0.2.0"

/* --- KEY DEFINITIONS --- */

typedef enum {
    KEY_NONE = 0,
    KEY_UP = 1000,
    KEY_DOWN,
    KEY_PAGE_UP,
    KEY_PAGE_DOWN,
    KEY_HALF_UP,
    KEY_HALF_DOWN,
    KEY_HOME,
    KEY_END,
    KEY_SEARCH_FORWARD = '/',
    KEY_SEARCH_BACKWARD = '?',
    KEY_SEARCH_NEXT = 'n',
    KEY_SEARCH_PREV = 'N',
    KEY_HELP = 'h',
    KEY_ESC = 27,
    KEY_QUIT = 'q',
} InkKey;

/* --- STRUCTURES --- */

typedef struct {
    int rows;
    int cols;
    int tty_fd;
    struct termios orig_termios;
    bool is_active;
} TerminalState;

/* --- MEMORY WRAPPERS --- */

void *xmalloc(size_t size);
void *xrealloc(void *ptr, size_t size);
char *xstrdup(const char *s);

typedef struct {
    char **raw_lines;
    size_t line_count;
    size_t line_cap;
} Document;

typedef struct {
    char **display_lines;
    size_t count;
    size_t cap;
    size_t *raw_to_display;
    size_t *display_to_raw;
} Layout;

typedef struct AppState {
    TerminalState ts;
    Document doc;
    Layout layout;
    int scroll_y;
    const char **filenames;
    int num_files;
    int current_file_index;
    volatile sig_atomic_t resize_pending;
    char last_pattern[256];
    int last_search_dir;
    bool search_failed;
    bool search_wrapped;
    bool show_help;
    bool show_line_numbers;
    bool running;
} AppState;

typedef struct {
    char *data;
    size_t len;
    size_t cap;
} RenderBuf;

/* --- MODULES --- */

/* app.c */
void app_init(AppState *app, int num_files, const char **filenames);
void app_run(AppState *app);
void app_cleanup(AppState *app);
void app_switch_file(AppState *app, int index);
extern AppState *g_app;

/* terminal.c */
void terminal_setup(TerminalState *ts);
void terminal_restore(TerminalState *ts);
void terminal_get_size(TerminalState *ts);
void terminal_enter_alt_buffer(void);
void terminal_exit_alt_buffer(void);
void terminal_hide_cursor(void);
void terminal_show_cursor(void);
void terminal_clear(void);

/* document.c */
void doc_init(Document *doc);
void doc_load_file(Document *doc, const char *filename);
void doc_load_stream(Document *doc, FILE *stream);
void doc_free(Document *doc);

/* layout.c */
void layout_init(Layout *layout);
void layout_compute(Layout *layout, Document *doc, int cols);
void layout_free(Layout *layout);

/* input.c */
int input_read_key(void);

/* view.c */
void rb_init(RenderBuf *rb);
void rb_append(RenderBuf *rb, const char *s, size_t len);
void rb_printf(RenderBuf *rb, const char *fmt, ...);
void rb_flush(RenderBuf *rb);
void rb_free(RenderBuf *rb);
void view_render_screen(AppState *app);
void view_read_prompt(AppState *app, char prompt_char, char *buf, size_t size);

/* commands.c */
void command_dispatch(AppState *app, int key);
void cmd_nav_up(AppState *app);
void cmd_nav_down(AppState *app);
void cmd_nav_page_up(AppState *app);
void cmd_nav_page_down(AppState *app);
void cmd_nav_half_up(AppState *app);
void cmd_nav_half_down(AppState *app);
void cmd_nav_home(AppState *app);
void cmd_nav_end(AppState *app);
void cmd_search_forward(AppState *app);
void cmd_search_backward(AppState *app);
void cmd_search_next(AppState *app);
void cmd_search_prev(AppState *app);
void cmd_sys_quit(AppState *app);
void cmd_sys_help(AppState *app);
void cmd_sys_colon(AppState *app);

/* utils.c */
void ink_die(const char *fmt, ...);
void utils_do_search(AppState *app, const char *pattern, int dir);
const char **utils_get_help_lines(int *count);

#endif
