#include "shocked/generated/shocked_cmake_config.h"

#include <curses.h>

#include <stdio.h>

/* keyboard usage */
int main(int argc, char *argv[]) {
    WINDOW* window = initscr(); 

    raw();
    keypad(stdscr, TRUE);
    noecho();
    printw("shocked version %u.%u.%u\n", SHOCKED_VERSION_MAJOR, SHOCKED_VERSION_MINOR, SHOCKED_VERSION_PATCH);


    int ch;
    do {
        ch = getch();
        printw("The key pressed is ");
        attron(A_BOLD);
        printw("%c", ch);
        attroff(A_BOLD);
    } while (ch != 'q');

    refresh();
    endwin();

    return 0;
}