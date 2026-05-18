#include "inkless.h"

AppState *g_app = NULL;

static void handle_sigint(int sig) {
    (void)sig;
    if (g_app) {
        app_cleanup(g_app);
    }
    exit(0);
}

static void handle_sigwinch(int sig) {
    (void)sig;
    if (g_app) {
        g_app->resize_pending = 1;
    }
}

void app_init(AppState *app, int num_files, const char **filenames) {
    app->filenames = filenames;
    app->num_files = num_files;
    app->current_file_index = 0;
    
    doc_init(&app->doc);
    layout_init(&app->layout);

    app->scroll_y = 0;
    app->resize_pending = 0;
    app->last_pattern[0] = '\0';
    app->last_search_dir = 1;
    app->search_failed = false;
    app->search_wrapped = false;
    app->search_case_insensitive = true; // Default to case-insensitive
    app->show_help = false;
    app->show_line_numbers = false;
    app->running = true;

    g_app = app;
    terminal_setup(&app->ts);
    app_switch_file(app, 0);
}

void app_switch_file(AppState *app, int index) {
    if (app->num_files > 0) {
        if (index < 0 || index >= app->num_files) return;
    } else if (index != 0) {
        return;
    }

    doc_free(&app->doc);
    layout_free(&app->layout);
    
    doc_init(&app->doc);
    layout_init(&app->layout);
    
    if (app->num_files > 0) {
        doc_load_file(&app->doc, app->filenames[index]);
    } else {
        doc_load_stream(&app->doc, stdin);
    }
    
    layout_compute(&app->layout, &app->doc, app->ts.cols);
    
    app->current_file_index = index;
    app->scroll_y = 0;
}

void app_run(AppState *app) {
    signal(SIGINT, handle_sigint);
    signal(SIGWINCH, handle_sigwinch);

    while (app->running) {
        if (app->resize_pending) {
            terminal_get_size(&app->ts);
            layout_compute(&app->layout, &app->doc, app->ts.cols);
            app->resize_pending = 0;
        }

        view_render_screen(app);

        int key = input_read_key();
        if (key != KEY_NONE) {
            command_dispatch(app, key);
        }
    }
}

void app_cleanup(AppState *app) {
    terminal_restore(&app->ts);
    doc_free(&app->doc);
    layout_free(&app->layout);
    g_app = NULL;
}
