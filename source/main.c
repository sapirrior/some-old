#include "ink.h"

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <filename> [filename...]\n", argv[0]);
        return 1;
    }

    AppState app;
    app_init(&app, argc - 1, (const char **)(argv + 1));
    app_run(&app);
    app_cleanup(&app);

    return 0;
}
