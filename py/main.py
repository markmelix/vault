from curses import *
import time

parts_of_start_menu = ["Existing storage file", "New storage file", "Exit"]


# Making start menu
def start_menu(stdscr, selected_row_idx):
    stdscr.clear()
    h, w = stdscr.getmaxyx()
    for idx, row in enumerate(parts_of_start_menu):
        global x
        global y
        x = w // 2 - len(row) // 2
        y = h // 2 - len(parts_of_start_menu) // 2 + idx
        if idx == selected_row_idx:
            stdscr.addstr(y, x, row, color_pair(2))
        else:
            stdscr.addstr(y, x, row)
        stdscr.refresh()
    x = w // 2 - len("VAULT") // 2
    y -= 7
    stdscr.addstr(y, x, "VAULT", A_UNDERLINE)


# Menu for creating new user
def new_file_menu(stdscr):
    stdscr.clear()
    echo()
    h, w = stdscr.getmaxyx()
    x_new_file = w // 2 - 10
    y_new_file = h // 2 - 2
    while True:
        stdscr.addstr(y_new_file, x_new_file, "Enter a new file name:")
        new_filename = stdscr.getstr(y_new_file + 1, x_new_file).decode("utf-8")
        if new_filename == '':
            stdscr.addstr(y_new_file + 2, x_new_file, "You didn't enter a file name!")
            stdscr.refresh()
            time.sleep(1.5)
            stdscr.clear()
            continue
        else:
            stdscr.addstr(y_new_file + 3, x_new_file, "{} - this is new file name".format(new_filename))
            stdscr.getch()
            break

    stdscr.refresh()


# The main function in which all the work of the program
def mainfunc(stdscr):
    init_pair(2, COLOR_BLACK, COLOR_WHITE)
    curs_set(False)
    stdscr.keypad(True)
    current_row_idx = 0
    while True:
        start_menu(stdscr, current_row_idx)
        key = stdscr.getch()
        if key == KEY_UP and current_row_idx > 0:
            current_row_idx -= 1
        elif key == KEY_DOWN and current_row_idx < len(parts_of_start_menu) - 1:
            current_row_idx += 1
        elif key in [10, 13]:
            if parts_of_start_menu[current_row_idx] == "Existing storage file":
                stdscr.clear()
                stdscr.addstr("Enter file name and decryption key")
                stdscr.refresh()
                stdscr.getch()
            elif parts_of_start_menu[current_row_idx] == "New storage file":
                new_file_menu(stdscr)
            elif parts_of_start_menu[current_row_idx] == "Exit":
                break
    endwin()


wrapper(mainfunc)
