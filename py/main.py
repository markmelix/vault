from curses import *
import time

parts_of_start_menu = ["Existing storage file", "New storage file", "Exit"]
parts_of_change_name_menu = ["Leave the name", "Change the name"]


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
        stdscr.clear()
        stdscr.addstr(y_new_file, x_new_file, "Enter a new file name:")
        new_filename = stdscr.getstr(y_new_file + 1, x_new_file).decode("utf-8")
        # If the user left a field empty
        if new_filename == '':
            stdscr.addstr(y_new_file + 2, x_new_file, "You didn't enter a file name!")
            stdscr.refresh()
            time.sleep(1)
            stdscr.clear()
            continue
        else:
            stdscr.clear()
            selected_row_idx = 0
            stdscr.addstr(h // 2 - len(parts_of_change_name_menu) // 2 - 2, w // 2 - len("Name of a new file is {}. Would you like to change it?".format(new_filename)) // 2, "Name of a new file is {}. Would you like to change it?".format(new_filename))
            # Leave or change the name of a new file
            while True:
                for idx, row in enumerate(parts_of_change_name_menu):
                    x_save_name = w // 2 - len(row) // 2
                    y_save_name = h // 2 - len(parts_of_change_name_menu) // 2 + idx
                    if idx == selected_row_idx:
                        stdscr.addstr(y_save_name, x_save_name, row, color_pair(2))
                    else:
                        stdscr.addstr(y_save_name, x_save_name, row)
                    stdscr.refresh()
                key = stdscr.getch()
                if key == KEY_UP and selected_row_idx > 0:
                    selected_row_idx -= 1
                elif key == KEY_DOWN and selected_row_idx < len(parts_of_change_name_menu) - 1:
                    selected_row_idx += 1
                elif key in [10, 13]:
                    if parts_of_change_name_menu[selected_row_idx] == "Leave the name":
                        stdscr.clear()
                        stdscr.addstr(y_new_file + 2, x_new_file, "The name is saved. Name of the new file is {}.".format(new_filename))
                        stdscr.refresh()
                        # Is the name of a new file saved or not
                        saving_the_name = 1
                        time.sleep(1)
                        break
                    elif parts_of_change_name_menu[selected_row_idx] == "Change the name":
                        saving_the_name = 0
                        break
            if saving_the_name == 1:
                break
            else:
                continue

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
                stdscr.addstr("Enter a file name and a decryption key")
                stdscr.refresh()
                stdscr.getch()
            elif parts_of_start_menu[current_row_idx] == "New storage file":
                new_file_menu(stdscr)
            elif parts_of_start_menu[current_row_idx] == "Exit":
                break
    endwin()


wrapper(mainfunc)
