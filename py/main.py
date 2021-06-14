from curses import *
import time
import os
import sys

parts_of_start_menu = ["Existing storage file", "New storage file", "Exit"]
parts_of_change_name_menu = ["Leave the path", "Change the path"]
come_to_start_menu = ["Come to start menu"]


# User input in the menu of the new/existing file
def user_input(stdscr):
    global path_of_the_file
    h, w = stdscr.getmaxyx()
    path_of_the_file = ''
    stdscr.clear()
    stdscr.addstr(h // 2 - 10, 1, "Enter the relative path of the file(including name of the file):")
    while True:
        ch = stdscr.getch()
        # Enter
        if ch in [10, 13]:
            break
        # Tab(go to the start menu)
        elif ch in [9]:
            mainfunc(stdscr)
        # Backspace
        elif ch in ['KEY_BACKSPACE', '\b', '\x7f', 263]:
            path_of_the_file = path_of_the_file[:-1]
            stdscr.clear()
            stdscr.addstr(h // 2 - 10, 1, "Enter the relative path of the file(including name of the file):")
            stdscr.addstr(h // 2 - 9, 1, path_of_the_file)
            stdscr.refresh()
            continue
        else:
            path_of_the_file = path_of_the_file + chr(ch)
            stdscr.clear()
            stdscr.addstr(h // 2 - 10, 1, "Enter the relative path of the file(including name of the file):")
            stdscr.addstr(h // 2 - 9, 1, path_of_the_file)
            stdscr.refresh()
            continue


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


# Menu for creating new file
def new_file_menu(stdscr):
    h, w = stdscr.getmaxyx()
    x_file = w // 2 - 10
    y_file = h // 2 - 2
    echo()
    while True:
        user_input(stdscr)
        # If the user left a field empty
        if path_of_the_file == '':
            stdscr.addstr(y_file - 7, 1,
                          "You didn't enter the file path!")
            stdscr.refresh()
            time.sleep(1)
            stdscr.clear()
            continue
        else:
            stdscr.clear()
            selected_row_idx = 0
            stdscr.addstr(h // 2 - len(parts_of_change_name_menu) // 2 - 2, w // 2 - len("New file path is {}. Would you like to change it?".format(
                path_of_the_file)) // 2, "New file path is {}. Would you like to change it?".format(path_of_the_file))
            # Leave or change the path of a new file
            while True:
                for idx, row in enumerate(parts_of_change_name_menu):
                    x_save_name = w // 2 - len(row) // 2
                    y_save_name = h // 2 - \
                        len(parts_of_change_name_menu) // 2 + idx
                    if idx == selected_row_idx:
                        stdscr.addstr(y_save_name, x_save_name,
                                      row, color_pair(2))
                    else:
                        stdscr.addstr(y_save_name, x_save_name, row)
                    stdscr.refresh()
                key = stdscr.getch()
                if key == KEY_UP and selected_row_idx > 0:
                    selected_row_idx -= 1
                elif key == KEY_DOWN and selected_row_idx < len(parts_of_change_name_menu) - 1:
                    selected_row_idx += 1
                elif key in [10, 13]:
                    if parts_of_change_name_menu[selected_row_idx] == "Leave the path":
                        stdscr.clear()
                        stdscr.addstr(h // 2 - len(parts_of_change_name_menu) // 2 - 2, w // 2 - len(
                            "The path is saved. New file path is {}.".format(path_of_the_file)) // 2, "The path is saved. New file path is {}.".format(path_of_the_file))
                        stdscr.refresh()
                        # Is the name of a new file saved or not
                        saving_the_path = 1
                        time.sleep(1)
                        break
                    elif parts_of_change_name_menu[selected_row_idx] == "Change the path":
                        saving_the_path = 0
                        break
            if saving_the_path == 1:
                break
            else:
                continue

    stdscr.refresh()


def existing_file_menu(stdscr):
    h, w = stdscr.getmaxyx()
    x_file = w // 2 - 10
    y_file = h // 2 - 2
    echo()
    while True:
        user_input(stdscr)
        if path_of_the_file == '':
            stdscr.addstr(y_file - 7, 1,
                          "You didn't enter the file path!")
            stdscr.refresh()
            time.sleep(1)
            stdscr.clear()
            continue
        elif path_of_the_file == "Leave":
            mainfunc(stdscr)
            break
        else:
            if os.path.isfile(path_of_the_file):
                stdscr.clear()
                stdscr.addstr(y_file, x_file, "Enter a description key:")
                stdscr.refresh()
                description_key = stdscr.getstr(
                    y_file - 1, x_file).decode("utf-8")
                break
            else:
                stdscr.addstr(y_file - 7, 1, "Path is not found!")
                stdscr.refresh()
                time.sleep(1)
                stdscr.clear()
                continue


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
                existing_file_menu(stdscr)
            elif parts_of_start_menu[current_row_idx] == "New storage file":
                new_file_menu(stdscr)
            elif parts_of_start_menu[current_row_idx] == "Exit":
                sys.exit()


wrapper(mainfunc)
