#include "inkless.h"

void doc_init(Document *doc) {
    doc->raw_lines = NULL;
    doc->line_count = 0;
    doc->line_cap = 0;
}

void doc_load_file(Document *doc, const char *filename) {
    FILE *f = fopen(filename, "r");
    if (!f) ink_die("Could not open file: %s", filename);

    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    while ((read = getline(&line, &len, f)) != -1) {
        if (doc->line_count >= doc->line_cap) {
            size_t new_cap = doc->line_cap == 0 ? 128 : doc->line_cap * 2;
            doc->raw_lines = xrealloc(doc->raw_lines, sizeof(char *) * new_cap);
            doc->line_cap = new_cap;
        }
        
        // Remove newline character if present
        if (read > 0 && line[read - 1] == '\n') {
            line[read - 1] = '\0';
            read--;
        }
        // Handle CRLF
        if (read > 0 && line[read - 1] == '\r') {
            line[read - 1] = '\0';
        }
        
        doc->raw_lines[doc->line_count++] = xstrdup(line);
    }
    free(line);
    fclose(f);
}

void doc_free(Document *doc) {
    if (doc->raw_lines) {
        for (size_t i = 0; i < doc->line_count; i++) {
            free(doc->raw_lines[i]);
        }
        free(doc->raw_lines);
    }
    doc_init(doc);
}
