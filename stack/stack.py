import logging
import os
from dataclasses import dataclass
from functools import lru_cache

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "WARNING"))


class Stack(tuple):
    def __str__(self):
        return f"{''.join(self)}"

    def with_push(self, next_char):
        return Stack(self + tuple([next_char]))

    def pop_all_cost(self):
        return len(self)


class States(dict):
    def __init__(self, data):
        for key in data.keys():
            assert isinstance(key, Stack), key
        return super().__init__(data)

    def __str__(self):
        data = {str(key): val for key, val in self.items()}
        return f"States({data})"

    def __hash__(self):
        return hash(tuple(sorted(self.items())))

    def copy(self):
        return States(super().copy())

    def min_max_remaining_cost(self, remaining_chars):
        cost_and_pop = min(cost + len(state) for state, cost in self.items())
        push = 3 * remaining_chars
        return cost_and_pop + push


class StateStore(dict):
    def __init__(self, string):
        self.total_len = len(string)
        # Fill with trivial states
        data = {
            idx: States({Stack([string[idx]]): idx * 3 + 2})
            for idx in range(0, len(string))
        }
        data[len(string)] = States({Stack(): len(string) * 3})
        for val in data.values():
            assert isinstance(val, States), val
        return super().__init__(data)

    def __str__(self):
        return f"StateStore({super().__str__()})"

    def add_state(self, idx: int, state: Stack, cost: int):
        assert isinstance(state, Stack)

        existing_cost = self[idx].get(state)
        if existing_cost is not None and existing_cost < cost:
            return

        remaining_chars = self.total_len - idx - 1
        # pop all and print all
        best_case = len(state) + remaining_chars
        min_max = self[idx].min_max_remaining_cost(remaining_chars)
        if best_case > min_max:
            logging.warning(f"skipping {idx=} {state=} {cost=}")
            return

        self[idx][state] = cost


def stack_dp(string):
    state_store = StateStore(string)
    logging.info(f"{state_store=}")
    for next_idx in range(1, len(string)):
        starting_states = state_store[next_idx - 1].copy()
        logging.info(
            f"starting_states({string[0:next_idx]} + {string[next_idx]})={starting_states}"
        )
        for starting_state, starting_cost in starting_states.items():
            if starting_state[-1] == string[next_idx]:
                # print
                state_store.add_state(next_idx, starting_state, starting_cost + 1)
            else:
                # push
                push_state = starting_state.with_push(string[next_idx])
                state_store.add_state(next_idx, push_state, starting_cost + 2)
                # pops
                pop_state = list(starting_state)
                next_cost = 1  # for the print
                while True:
                    try:
                        pop_state.pop()
                    except IndexError:
                        break
                    next_cost += 1  # for the pop
                    if pop_state and pop_state[-1] == string[next_idx]:
                        # print
                        next_pop_state = Stack(pop_state)
                    else:
                        # push
                        next_pop_state = Stack(pop_state).with_push(string[next_idx])
                        next_cost += 1  # for the push
                    state_store.add_state(
                        next_idx, next_pop_state, starting_cost + next_cost
                    )
    # Remove all
    next_idx = len(string)
    ending_states = state_store[next_idx - 1].copy()
    logging.info(f"ending_states({string[0:next_idx]})={ending_states}")
    for ending_state, ending_cost in ending_states.items():
        pop_state = list(ending_state)
        next_cost = 0
        while True:
            try:
                pop_state.pop()
            except IndexError:
                break
            next_cost += 1  # for the pop
            add_pop_state = Stack(pop_state)
            state_store.add_state(next_idx, add_pop_state, ending_cost + next_cost)
    return state_store[len(string)][tuple()]


class CaptureEq:
    """Object wrapper that remembers "other" for successful equality tests."""

    def __init__(self, obj):
        self.obj = obj
        self.match = obj

    def __eq__(self, other):
        result = self.obj == other
        if result:
            self.match = other
        return result

    def __hash__(self):
        return hash(self.obj)


def stack_astar(string):
    from heapq import heappush, heappop

    @dataclass(frozen=True)
    class Node:
        characters_printed: int = 0
        stack: Stack = Stack()
        cheapest_operation_count: int = 0

        @property
        def best_case_operation_count(self):
            """
            Current cheapest
            pop the rest of the stack
            print the remaining characters
            """
            remaining_pop_operation = len(self.stack)
            remaining_chars_to_print = [c for c in string[self.characters_printed :]]
            chars_printable_while_popping = [
                c for c in remaining_chars_to_print if c in self.stack
            ]
            chars_printable_after_popping = [
                c for c in remaining_chars_to_print if c not in self.stack
            ]
            set_chars_printable_after_popping = set(chars_printable_after_popping)
            return (
                self.cheapest_operation_count  # previous
                + remaining_pop_operation  # pop
                + len(chars_printable_while_popping)  # print while pop
                + len(set_chars_printable_after_popping)  # push
                + len(chars_printable_after_popping)  # print
                + len(set_chars_printable_after_popping)  # pop
            )

        def __lt__(self, other) -> bool:
            """
            For heap queue to work
            """
            return self.best_case_operation_count < other.best_case_operation_count

        def __hash__(self) -> int:
            return hash((self.characters_printed, self.stack))

        def __repr__(self):
            return f"N({self.characters_printed}, {self.stack})"

        def is_goal(self):
            return self.stack == Stack() and self.characters_printed == len(string)

        def get_neighbours(self):
            try:
                next_char = string[self.characters_printed]
            except IndexError:
                if self.stack:  # pop
                    return [
                        Node(
                            characters_printed=self.characters_printed,
                            stack=Stack(self.stack[:-1]),
                            cheapest_operation_count=self.cheapest_operation_count + 1,
                        )
                    ]
                else:
                    raise NotImplementedError()

            neighbours = []
            if self.stack and self.stack[-1] == next_char:  # print
                neighbours.append(
                    Node(
                        characters_printed=self.characters_printed + 1,
                        stack=self.stack,
                        cheapest_operation_count=self.cheapest_operation_count + 1,
                    )
                )
            else:  # push
                neighbours.append(
                    Node(
                        characters_printed=self.characters_printed,
                        stack=Stack([*self.stack, next_char]),
                        cheapest_operation_count=self.cheapest_operation_count + 1,
                    )
                )
                if self.stack:  # pop
                    neighbours.append(
                        Node(
                            characters_printed=self.characters_printed,
                            stack=Stack(self.stack[:-1]),
                            cheapest_operation_count=self.cheapest_operation_count + 1,
                        )
                    )
            return neighbours

    discovered_nodes_heap = []
    discovered_nodes_set = set()
    start_node = Node()
    heappush(discovered_nodes_heap, start_node)
    discovered_nodes_set.add(start_node)
    while True:
        logging.debug(f"{discovered_nodes_set=}")
        current = heappop(discovered_nodes_heap)
        discovered_nodes_set.remove(current)
        if current.is_goal():
            return current.cheapest_operation_count
        for neighbour in current.get_neighbours():
            capture = CaptureEq(neighbour)
            if capture in discovered_nodes_set:
                if (
                    capture.match.cheapest_operation_count
                    <= neighbour.cheapest_operation_count
                ):
                    continue
                heappop(capture.match)
                discovered_nodes_set.remove(capture.match)
            heappush(discovered_nodes_heap, neighbour)
            discovered_nodes_set.add(neighbour)


def main():
    n = int(input())
    strings = [input() for _ in range(n)]
    for string in strings:
        print(stack_astar(string))


def test():
    print(stack_astar("abba"))
    print(stack_astar("rollover ahead"))
    print(stack_astar("abba rollover ahead ogopogo spotted!"))


if __name__ == "__main__":
    # test()
    main()
