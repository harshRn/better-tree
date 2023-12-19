#!/usr/bin/env python
"""
My own implementation of linux `tree` command.
"""

import os
import sys
from pathlib import Path


class Tree:
    def __init__(self, path: str = None) -> None:
        self.path = path if path else os.getcwd()
        self.inside = True

    def print_file(self, entry: os.DirEntry, path_files: list, tabs: int = 0) -> None:
        if tabs > 2:
            print("   │", end="")
        if tabs > 0 and self.inside:
            print("│", end="")
        print(" " * tabs, end="")
        if entry.name != path_files[-1]:
            print("├─", end=" ")
        else:
            print("└─", end=" ")
        print(entry.name)

    def print_dir(self, entry: os.DirEntry, path_files: list, tabs: int = 0) -> None:
        if tabs > 0 and self.inside:
            print("│", end="")
        print(" " * tabs, end="")
        if entry.name != path_files[-1]:
            print("├─", end=" ")
        else:
            self.inside = False
            print("└─", end=" ")
        print(entry.name)
        print("│", end="\r")
        self.walk(entry.path, tabs + 2)

    def print_root(self, directory, path_files: int, tabs: int = 0) -> None:
        for entry in directory:
            if entry.name.startswith("."):
                continue
            if entry.is_file():
                self.print_file(entry, path_files, tabs)
            elif entry.is_dir():
                self.print_dir(entry, path_files, tabs)

    def walk(self, path=None, tabs: int = 0):
        p = path
        if not p:
            p = self.path
        if not Path(p).exists():
            raise
        with os.scandir(p) as directory:
            path_files = os.listdir(p)
            self.print_root(directory, path_files, tabs)


if __name__ == "__main__":
    if len(sys.argv) <= 2:
        path = sys.argv[1] if len(sys.argv) > 1 else "."
        if Path(path).exists():
            print(path)
            tree = Tree()
            tree.walk()
    else:
        print("Usage: tree <dir>")
