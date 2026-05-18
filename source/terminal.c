#include "inkless.h"
#include <unistd.h>
#include <sys/ioctl.h>

void terminal_setup(TerminalState *ts) {
    ts->is_active = false;
    if (tcgetattr(STDIN_FILENO, &ts->orig_termios) == -1) {
        ink_die("Terminal setup failed: tcgetattr");
    }

    struct termios raw = ts->orig_termios;
    raw.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    raw.c_oflag &= ~(OPOST);
    raw.c_cflag |= (CS8);
    raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;

    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1) {
        ink_die("Terminal setup failed: tcsetattr");
    }

    ts->is_active = true;
    terminal_enter_alt_buffer();
    terminal_hide_cursor();
    terminal_get_size(ts);
}

void terminal_restore(TerminalState *ts) {
    if (!ts->is_active) return;
    terminal_exit_alt_buffer();
    terminal_show_cursor();
    tcsetattr(STDIN_FILENO, TCSAFLUSH, &ts->orig_termios);
    ts->is_active = false;
}

void terminal_get_size(TerminalState *ts) {
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
        ts->rows = 24;
        ts->cols = 80;
    } else {
        ts->rows = ws.ws_row;
        ts->cols = ws.ws_col;
    }
}

void terminal_enter_alt_buffer(void) {
    const char *s = "\x1b[?1049h";
    (void)write(STDOUT_FILENO, s, strlen(s));
}

void terminal_exit_alt_buffer(void) {
    const char *s = "\x1b[?1049l";
    (void)write(STDOUT_FILENO, s, strlen(s));
}

void terminal_hide_cursor(void) {
    const char *s = "\x1b[?25l";
    (void)write(STDOUT_FILENO, s, strlen(s));
}

void terminal_show_cursor(void) {
    const char *s = "\x1b[?25h";
    (void)write(STDOUT_FILENO, s, strlen(s));
}

void terminal_clear(void) {
    const char *s = "\x1b[2J\x1b[H";
    (void)write(STDOUT_FILENO, s, strlen(s));
}
