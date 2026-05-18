#include "inkless.h"
#include <regex.h>

static const char *help_lines[] = {
    "",
    "  Navigation",
    "    j, Down          Move down one line",
    "    k, Up            Move up one line",
    "    f, Space         Page down",
    "    b                Page up",
    "    d                Half-page down",
    "    u                Half-page up",
    "    g, <, Home       Jump to beginning",
    "    G, >, End        Jump to end",
    "    :n               Next file",
    "    :p               Previous file",
    "    :<number>        Jump to line number",
    "    :N               Toggle line numbers",
    "",
    "  Search",
    "    /                Search forward (Regex)",
    "    ?                Search backward (Regex)",
    "    n                Repeat search forward",
    "    N                Repeat search backward",
    "    :i               Toggle case-insensitive search",
    "    Esc              Clear active search highlights",
    "",
    "  System",
    "    h                Toggle this help menu",
    "    q                Quit Ink",
    "",
    "  Press Esc or h to return to the document..."
};

const char **utils_get_help_lines(int *count) {
    if (count) *count = sizeof(help_lines) / sizeof(help_lines[0]);
    return help_lines;
}

/* --- MEMORY WRAPPERS --- */

void *xmalloc(size_t size) {
    void *ptr = malloc(size);
    if (!ptr) ink_die("Out of memory (malloc failed)");
    return ptr;
}

void *xrealloc(void *ptr, size_t size) {
    void *new_ptr = realloc(ptr, size);
    if (!new_ptr && size > 0) ink_die("Out of memory (realloc failed)");
    return new_ptr;
}

char *xstrdup(const char *s) {
    char *dup = strdup(s);
    if (!dup) ink_die("Out of memory (strdup failed)");
    return dup;
}

/* --- ERROR HANDLING --- */

void ink_die(const char *fmt, ...) {
    if (g_app) {
        terminal_restore(&g_app->ts);
    }
    
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    fprintf(stderr, "\n");
    exit(1);
}

void utils_do_search(AppState *app, const char *pattern, int dir) {
    if (!pattern || !pattern[0] || app->doc.line_count == 0) return;
    app->search_failed = false;
    app->search_wrapped = false;

    int flags = REG_EXTENDED;
    if (app->search_case_insensitive) flags |= REG_ICASE;

    regex_t regex;
    if (regcomp(&regex, pattern, flags) != 0) {
        app->search_failed = true;
        return;
    }

    int current_raw = 0;
    for (size_t i = 0; i < app->doc.line_count; i++) {
        if (app->layout.raw_to_display[i] <= (size_t)app->scroll_y) current_raw = (int)i;
        else break;
    }

    int found = -1;
    bool wrapped = false;
    int n = (int)app->doc.line_count;
    for (int i = 1; i <= n; i++) {
        int idx = (current_raw + (i * dir) + n) % n;
        
        if (dir > 0 && idx < current_raw) wrapped = true;
        if (dir < 0 && idx > current_raw) wrapped = true;

        if (regexec(&regex, app->doc.raw_lines[idx], 0, NULL, 0) == 0) {
            found = idx;
            break;
        }
    }

    if (found != -1) {
        app->scroll_y = (int)app->layout.raw_to_display[found];
        app->search_wrapped = wrapped;
        strncpy(app->last_pattern, pattern, sizeof(app->last_pattern) - 1);
        app->last_pattern[sizeof(app->last_pattern) - 1] = '\0';
    } else {
        app->search_failed = true;
    }

    regfree(&regex);
}
