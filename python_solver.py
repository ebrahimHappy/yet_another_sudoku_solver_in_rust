from itertools import chain

import numpy as np


BOARD_SIZE = 9
BLOCK_SIZE = 3


def row_id(cell_id):
    return cell_id // BOARD_SIZE


def col_id(cell_id):
    return cell_id % BOARD_SIZE


def block_id(cell_id):
    i = row_id(cell_id) // BLOCK_SIZE
    j = col_id(cell_id) // BLOCK_SIZE
    return i * BLOCK_SIZE + j


def row_members(row_id):
    return range(row_id * BOARD_SIZE, (row_id+1) * BOARD_SIZE)


def col_members(col_id):
    return range(col_id, BOARD_SIZE ** 2, BOARD_SIZE)


def block_members(block_id):
    ii, jj = block_id // BLOCK_SIZE, block_id % BLOCK_SIZE
    return ((ii*BLOCK_SIZE+i)*BOARD_SIZE + (jj*BLOCK_SIZE+j)
            for i in range(3) for j in range(3))


def neighbors(cell_id):
    return list(set(chain(
        row_members(row_id(cell_id)),
        col_members(col_id(cell_id)),
        block_members(block_id(cell_id)),
    )) - {cell_id})


class Unsolvable(Exception):
    pass


class Board:
    def __init__(self):
        self.values = {}
        self.possibility = np.ones((BOARD_SIZE**2, BOARD_SIZE), dtype='bool')
        self.value_counts = np.ones((BOARD_SIZE**2), dtype='uint8') * BOARD_SIZE
        self.row_counts = (
            np.ones((BOARD_SIZE, BOARD_SIZE), dtype='uint8') * BOARD_SIZE)
        self.col_counts = (
            np.ones((BOARD_SIZE, BOARD_SIZE), dtype='uint8') * BOARD_SIZE)
        self.block_counts = (
            np.ones((BOARD_SIZE, BOARD_SIZE), dtype='uint8') * BOARD_SIZE)

    def unset(self, cell_id, value):
        if not self.possibility[cell_id, value]:
            return
        self.possibility[cell_id, value] = False
        self.value_counts[cell_id] -= 1
        self.row_counts[row_id(cell_id), value] -= 1
        self.col_counts[col_id(cell_id), value] -= 1
        self.block_counts[block_id(cell_id), value] -= 1

        if min(self.value_counts[cell_id],
               self.row_counts[row_id(cell_id), value],
               self.col_counts[col_id(cell_id), value],
               self.block_counts[block_id(cell_id), value],
               ) == 0:
            raise Unsolvable()

        if self.value_counts[cell_id] == 1:
            self.set(cell_id, np.where(self.possibility[cell_id])[0][0])

        if self.row_counts[row_id(cell_id), value] == 1:
            for neigh in row_members(row_id(cell_id)):
                if self.possibility[neigh, value]:
                    self.set(neigh, value)

        if self.col_counts[col_id(cell_id), value] == 1:
            for neigh in col_members(col_id(cell_id)):
                if self.possibility[neigh, value]:
                    self.set(neigh, value)

        if self.block_counts[block_id(cell_id), value] == 1:
            for neigh in block_members(block_id(cell_id)):
                if self.possibility[neigh, value]:
                    self.set(neigh, value)

    def set(self, cell_id, value):
        if cell_id in self.values:
            if self.values[cell_id] == value:
                return
            else:
                raise Unsolvable()
        self.values[cell_id] = value

        for i in range(BOARD_SIZE):
            if i != value:
                self.unset(cell_id, i)

        for neigh in neighbors(cell_id):
            self.unset(neigh, value)

    def show(self):
        for row in range(BOARD_SIZE):
            for cell in row_members(row):
                value = self.values.get(cell, None)
                if value is None:
                    print(' ', end='')
                else:
                    print(value+1, end='')
            print('')


board = Board()
with open('problems/easy.txt') as f:
    for row_id_, line in enumerate(f.readlines()):
        if row_id_ >= BOARD_SIZE:
            break
        for cell, value in zip(row_members(row_id_), line):
            value = int(value)
            if value > 0:
                board.set(cell, value-1)
                board.show()
                print()
