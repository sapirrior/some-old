#include "inkless.h"
#include <unistd.h>
#include <regex.h>

/* --- Render Buffer Implementation --- */

void rb_init(RenderBuf *rb) {
    rb->cap = 16384;
    rb->data = xmalloc(rb->cap);
    rb->len = 0;
    rb->data[0] = '\0';
}

void rb_append(RenderBuf *rb, const char *s, size_t len) {
    if (!rb->data) return;
    if (rb->len + len >= rb->cap) {
        size_t new_cap = (rb->cap + len) * 2;
        rb->data = xrealloc(rb->data, new_cap);
        rb->cap = new_cap;
    }
    memcpy(rb->data + rb->len, s, len);
    rb->len += len;
    rb->data[rb->len] = '\0';
}

void rb_printf(RenderBuf *rb, const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    
    va_list ap_copy;
    va_copy(ap_copy, ap);
    int n = vsnprintf(NULL, 0, fmt, ap_copy);
    va_end(ap_copy);

    if (n < 0) {
        va_end(ap);
        return;
    }

    size_t size = (size_t)n;
    char *buf = xmalloc(size + 1);
    vsnprintf(buf, size + 1, fmt, ap);
    rb_append(rb, buf, size);
    free(buf);
    va_end(ap);
}

void rb_flush(RenderBuf *rb) {
    if (rb->len > 0) {
        (void)write(STDOUT_FILENO, rb->data, rb->len);
        rb->len = 0;
        rb->data[0] = '\0';
    }
}

void rb_free(RenderBuf *rb) {
    free(rb->data);
    rb->data = NULL;
    rb->len = 0;
    rb->cap = 0;
}

/* --- Prompt Handling --- */

void view_read_prompt(AppState *app, char prompt_char, char *buf, size_t size) {
    size_t len = 0;
    buf[0] = '\0';
    terminal_show_cursor();

    RenderBuf rb;
    rb_init(&rb);

    while (1) {
        rb.len = 0;
        rb_printf(&rb, "\x1b[%d;1H\x1b[m\x1b[2K%c%s", app->ts.rows, prompt_char, buf);
        rb_flush(&rb);

        int c = input_read_key();
        if (c == '\r' || c == '\n') break;
        if (c == '\x1b') { len = 0; buf[0] = '\0'; break; }
        if (c == 127 || c == 8) { // Backspace
            if (len > 0) buf[--len] = '\0';
        } else if (c >= 32 && c < 127 && len < size - 1) {
            buf[len++] = (char)c;
            buf[len] = '\0';
        }
    }
    rb_free(&rb);
    terminal_hide_cursor();
}

/* --- Screen Rendering --- */

static void view_render_help(AppState *app, RenderBuf *rb) {
    int rows = app->ts.rows;
    int cols = app->ts.cols;
    
    rb_append(rb, "\x1b[m\x1b[2J\x1b[H", 10);
    
    char title[64];
    snprintf(title, sizeof(title), "--- Inkless Pager v%s Help ---", INKLESS_VERSION);
    int title_len = (int)strlen(title);
    int title_x = (cols - title_len) / 2;
    if (title_x < 1) title_x = 1;
    
    rb_printf(rb, "\x1b[1;%dH%s", title_x, title);
    
    int num_lines = 0;
    const char **help_lines = utils_get_help_lines(&num_lines);

    for (int i = 0; i < num_lines; i++) {
        if (i + 3 > rows) break;
        rb_printf(rb, "\x1b[%d;1H%s", i + 3, help_lines[i]);
    }
}

void view_render_screen(AppState *app) {
    RenderBuf rb;
    rb_init(&rb);

    if (app->show_help) {
        view_render_help(app, &rb);
        rb_flush(&rb);
        rb_free(&rb);
        return;
    }

    rb_append(&rb, "\x1b[m\x1b[H", 6);

    int margin = (app->ts.cols * 8) / 100;
    int view_height = app->ts.rows - 1;
    if (view_height < 0) view_height = 0;

    if (app->scroll_y < 0) app->scroll_y = 0;
    if (app->layout.count > 0) {
        if (app->scroll_y >= (int)app->layout.count) {
            app->scroll_y = (int)app->layout.count - 1;
        }
    } else {
        app->scroll_y = 0;
    }

    regex_t regex;
    bool has_regex = false;
    if (app->last_pattern[0] != '\0' && regcomp(&regex, app->last_pattern, REG_EXTENDED) == 0) {
        has_regex = true;
    }

    for (int i = 0; i < view_height; i++) {
        int line_idx = app->scroll_y + i;
        rb_printf(&rb, "\x1b[%d;1H\x1b[2K", i + 1);

        if (line_idx < (int)app->layout.count) {
            if (app->show_line_numbers) {
                size_t raw_line = app->layout.display_to_raw[line_idx];
                if (raw_line > 0) {
                    rb_printf(&rb, "\x1b[38;5;242m%*zu \x1b[m", margin - 1, raw_line);
                } else {
                    for (int j = 0; j < margin; j++) rb_append(&rb, " ", 1);
                }
            } else {
                for (int j = 0; j < margin; j++) rb_append(&rb, " ", 1);
            }
            
            const char *line = app->layout.display_lines[line_idx];
            if (!line) continue;

            if (has_regex) {
                regmatch_t pmatch;
                const char *ptr = line;
                while (regexec(&regex, ptr, 1, &pmatch, 0) == 0) {
                    size_t match_len = (size_t)(pmatch.rm_eo - pmatch.rm_so);
                    rb_append(&rb, ptr, (size_t)pmatch.rm_so);
                    rb_append(&rb, "\x1b[7m", 4);
                    rb_append(&rb, ptr + pmatch.rm_so, match_len);
                    rb_append(&rb, "\x1b[m", 3);
                    ptr += pmatch.rm_eo;
                    if (pmatch.rm_so == pmatch.rm_eo) {
                        if (*ptr == '\0') break;
                        rb_append(&rb, ptr, 1);
                        ptr++;
                    }
                    if (*ptr == '\0') break;
                }
                rb_append(&rb, ptr, strlen(ptr));
            } else {
                rb_append(&rb, line, strlen(line));
            }
        }
    }

    if (has_regex) regfree(&regex);

    rb_printf(&rb, "\x1b[%d;1H\x1b[2K", app->ts.rows);

    if (app->search_failed) {
        rb_append(&rb, "\x1b[7mPattern not found (press any key)\x1b[m", 38);
    } else {
        int current_last_line = app->scroll_y + view_height;
        if (current_last_line >= (int)app->layout.count) {
            rb_append(&rb, "\x1b[7m(END)\x1b[m", 13);
        } else {
            rb_append(&rb, ":", 1);
        }
    }

    rb_flush(&rb);
    rb_free(&rb);
}
