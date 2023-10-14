from curses import *
from vault import Vault
import vault as vlt
import time
import os
import sys

parts_of_start_menu = ["Existing storage file", "New storage file", "Exit"]
parts_of_change_name_menu = ["Leave the path", "Change the path"]
come_to_start_menu = ["Come to start menu"]


# User input in the menu of the new/existing file
def user_input(stdscr):
    global path_of_the_file
    global file_password
    global file_size
    h, w = stdscr.getmaxyx()
    path_of_the_file = ""
    file_size = ""
    file_password = ""
    stdscr.clear()
    stdscr.addstr(
        h // 2 - 10,
        1,
        "Enter the relative path of the file(including name of the file):",
    )
    while True:
        ch = stdscr.getch()
        # Enter
        if ch in [10, 13]:
            # If the user left a field empty
            if path_of_the_file == "":
                stdscr.addstr(h // 2 - 9, 1, "You didn't enter the file path!")
                stdscr.refresh()
                time.sleep(1)
                stdscr.clear()
                stdscr.addstr(
                    h // 2 - 10,
                    1,
                    "Enter the relative path of the file(including name of the file):",
                )
                continue
            else:
                break
        # Tab(go to the start menu)
        elif ch in [9]:
            mainfunc(stdscr)
        # Backspace
        elif ch in ["KEY_BACKSPACE", "\b", "\x7f", 263]:
            path_of_the_file = path_of_the_file[:-1]
            stdscr.clear()
            stdscr.addstr(
                h // 2 - 10,
                1,
                "Enter the relative path of the file(including name of the file):",
            )
            stdscr.addstr(h // 2 - 9, 1, path_of_the_file)
            stdscr.refresh()
            continue
        else:
            path_of_the_file = path_of_the_file + chr(ch)
            stdscr.clear()
            stdscr.addstr(
                h // 2 - 10,
                1,
                "Enter the relative path of the file(including name of the file):",
            )
            stdscr.addstr(h // 2 - 9, 1, path_of_the_file)
            stdscr.refresh()
            continue
    stdscr.clear()
    stdscr.addstr(
        h // 2 - 10,
        1,
        "Enter a size of value(it's highly recommended to use powers of 2 as value for this)."
        "To set default size(1024) just press enter:",
    )
    while True:
        ch = stdscr.getch()
        # Enter
        if ch in [10, 13]:
            # If the user left a field empty
            if file_size == "":
                file_size = 1024
                break
            else:
                try:
                    file_size = int(file_size)
                    break
                except:
                    stdscr.addstr(h // 2 - 9, 1, "You didn't enter integer!")
                    stdscr.refresh()
                    time.sleep(1)
                    stdscr.clear()
                    stdscr.addstr(
                        h // 2 - 10,
                        1,
                        "Enter a size of value(it's highly recommended to use powers of 2 as value for this)."
                        "To set default size(1024) just press enter:",
                    )
                    continue
        # Tab(go to the start menu)
        elif ch in [9]:
            mainfunc(stdscr)
        # Backspace
        elif ch in ["KEY_BACKSPACE", "\b", "\x7f", 263]:
            file_size = file_size[:-1]
            stdscr.clear()
            stdscr.addstr(
                h // 2 - 10,
                1,
                "Enter a size of value(it's highly recommended to use powers of 2 as value for this)."
                "To set default size(1024) just press enter:",
            )
            stdscr.addstr(h // 2 - 9, 1, file_size)
            stdscr.refresh()
            continue
        else:
            file_size = file_size + chr(ch)
            stdscr.clear()
            stdscr.addstr(
                h // 2 - 10,
                1,
                "Enter a size of value(it's highly recommended to use powers of 2 as value for this)."
                "To set default size(1024) just press enter:",
            )
            stdscr.addstr(h // 2 - 9, 1, file_size)
            stdscr.refresh()
            continue
    stdscr.clear()
    stdscr.addstr(
        h // 2 - 10, 1, "Enter a description key/vault password(remember it):"
    )
    while True:
        noecho()
        ch = stdscr.getch()
        # Enter
        if ch in [10, 13]:
            # If the user left a field empty
            if file_password == "":
                stdscr.addstr(h // 2 - 9, 1, "You didn't enter the file password!")
                stdscr.refresh()
                time.sleep(1)
                stdscr.clear()
                stdscr.addstr(
                    h // 2 - 10,
                    1,
                    "Enter a description key/vault password(remember it):",
                )
                continue
            else:
                break
        # Tab(go to the start menu)
        elif ch in [9]:
            mainfunc(stdscr)
        # Backspace
        elif ch in ["KEY_BACKSPACE", "\b", "\x7f", 263]:
            file_password = file_password[:-1]
            continue
        else:
            file_password = file_password + chr(ch)
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
    global path_of_the_file
    h, w = stdscr.getmaxyx()
    x_file = w // 2 - 10
    y_file = h // 2 - 2
    echo()
    while True:
        user_input(stdscr)
        path_of_the_file = path_of_the_file + ".bin"
        stdscr.clear()
        selected_row_idx = 0
        stdscr.addstr(
            h // 2 - len(parts_of_change_name_menu) // 2 - 2,
            w // 2
            - len(
                "New file path is {}. Would you like to change it?".format(
                    path_of_the_file
                )
            )
            // 2,
            "New file path is {}. Would you like to change it?".format(
                path_of_the_file
            ),
        )
        # Leave or change the path of a new file
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
            elif (
                key == KEY_DOWN
                and selected_row_idx < len(parts_of_change_name_menu) - 1
            ):
                selected_row_idx += 1
            elif key in [10, 13]:
                if parts_of_change_name_menu[selected_row_idx] == "Leave the path":
                    stdscr.clear()
                    stdscr.addstr(
                        h // 2 - len(parts_of_change_name_menu) // 2 - 2,
                        w // 2
                        - len(
                            "The path is saved. New file path is {}.".format(
                                path_of_the_file
                            )
                        )
                        // 2,
                        "The path is saved. New file path is {}.".format(
                            path_of_the_file
                        ),
                    )
                    stdscr.refresh()
                    # Is the name of a new file saved or not
                    saving_the_path = 1
                    time.sleep(1)
                    break
                elif parts_of_change_name_menu[selected_row_idx] == "Change the path":
                    saving_the_path = 0
                    break
        if saving_the_path == 1:
            echo()
            stdscr.clear()
            stdscr.addstr(
                h // 2,
                w // 2 - len("Write whatever you want") // 2,
                "Write whatever you want",
            )
            stdscr.refresh()
            time.sleep(1)
            stdscr.clear()
            vault_info = stdscr.getstr(1, 1)
            vault_info = vault_info.decode("utf-8", "replace")
            vault = Vault(file_password, vlt.bytes_to_bits(file_size))
            vault.encrypt_append(file_password, vault_info)
            vault.save(path_of_the_file)
            break
        else:
            continue
    stdscr.refresh()


def existing_file_menu(stdscr):
    global path_of_the_file
    h, w = stdscr.getmaxyx()
    x_file = w // 2 - 10
    y_file = h // 2 - 2
    echo()
    while True:
        stdscr.clear()
        stdscr.addstr(
            h // 2 - 10,
            1,
            "Enter the relative path of the file(including name of the file):",
        )
        path_of_the_file = ""
        while True:
            ch = stdscr.getch()
            # Enter
            if ch in [10, 13]:
                # If the user left a field empty
                if path_of_the_file == "":
                    stdscr.addstr(h // 2 - 9, 1, "You didn't enter the file path!")
                    stdscr.refresh()
                    time.sleep(1)
                    stdscr.clear()
                    stdscr.addstr(
                        h // 2 - 10,
                        1,
                        "Enter the relative path of the file(including name of the file):",
                    )
                    continue
                else:
                    break
            # Tab(go to the start menu)
            elif ch in [9]:
                mainfunc(stdscr)
            # Backspace
            elif ch in ["KEY_BACKSPACE", "\b", "\x7f", 263]:
                path_of_the_file = path_of_the_file[:-1]
                stdscr.clear()
                stdscr.addstr(
                    h // 2 - 10,
                    1,
                    "Enter the relative path of the file(including name of the file):",
                )
                stdscr.addstr(h // 2 - 9, 1, path_of_the_file)
                stdscr.refresh()
                continue
            else:
                path_of_the_file = path_of_the_file + chr(ch)
                stdscr.clear()
                stdscr.addstr(
                    h // 2 - 10,
                    1,
                    "Enter the relative path of the file(including name of the file):",
                )
                stdscr.addstr(h // 2 - 9, 1, path_of_the_file)
                stdscr.refresh()
                continue
        if os.path.isfile(path_of_the_file):
            noecho()
            stdscr.clear()
            stdscr.addstr(y_file, x_file, "Enter the description key/vault password:")
            stdscr.refresh()
            description_key = stdscr.getstr(y_file - 1, x_file).decode("utf-8")
            stdscr.clear()
            vault = Vault.open(path_of_the_file)
            stdscr.addstr(vault.decrypt(description_key))
            stdscr.addstr(
                h - 1, 1, "Add something - 'a'. Overwrite vault - 'o'. Exit - tab"
            )
            stdscr.refresh()
            key = stdscr.getch()
            if key == ord("a"):
                echo()
                stdscr.clear()
                vault_info = stdscr.getstr(1, 1)
                vault_info = vault_info.decode("utf-8", "replace")
                vault.encrypt_append(description_key, vault_info)
                vault.save(path_of_the_file)
                break
            elif key == ord("o"):
                echo()
                stdscr.clear()
                vault_info = stdscr.getstr(1, 1)
                vault_info = vault_info.decode("utf-8", "replace")
                vault.encrypt(description_key, vault_info)
                vault.save(path_of_the_file)
                break
            elif key in [9]:
                mainfunc(stdscr)
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
