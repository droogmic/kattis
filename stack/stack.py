import logging
import os

logging.basicConfig(level=os.environ.get("LOG_LEVEL", "WARNING"))


class States:
    def __init__(self):
        self.data = 


def stack(string):
    # Fill with trivial states
    states = {idx: {tuple([string[idx]]): idx * 3 + 2} for idx in range(0, len(string))}
    states[len(string)] = {tuple(): len(string) * 3 + 3}
    logging.info(f"{states=}")
    for next_idx in range(1, len(string)):
        starting_states = states[next_idx - 1].copy()
        print_starting_states = {"".join(s): c for s, c in starting_states.items()}
        logging.info(
            f"starting_states({string[0:next_idx]} + {string[next_idx]})={print_starting_states}"
        )
        for starting_state, starting_cost in starting_states.items():
            if starting_state[-1] == string[next_idx]:
                # print
                states[next_idx][starting_state] = min(
                    states[next_idx].get(starting_state, 1000), starting_cost + 1
                )
            else:
                # push
                push_state = tuple(starting_state + tuple([string[next_idx]]))
                states[next_idx][push_state] = min(
                    states[next_idx].get(push_state, 1000), starting_cost + 2
                )
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
                        next_pop_state = tuple(pop_state)
                    else:
                        # push
                        next_pop_state = tuple(pop_state + [string[next_idx]])
                        next_cost += 1  # for the push
                    states[next_idx][next_pop_state] = min(
                        states[next_idx].get(next_pop_state, 1000),
                        starting_cost + next_cost,
                    )
    # Remove all
    next_idx = len(string)
    ending_states = states[next_idx - 1].copy()
    print_ending_states = {"".join(s): c for s, c in ending_states.items()}
    logging.info(f"ending_states({string[0:next_idx]})={print_ending_states}")
    for ending_state, ending_cost in ending_states.items():
        pop_state = list(ending_state)
        next_cost = 0
        while True:
            try:
                pop_state.pop()
            except IndexError:
                break
            next_cost += 1  # for the pop
            add_pop_state = tuple(pop_state)
            states[next_idx][add_pop_state] = min(
                states[next_idx].get(add_pop_state, 1000), ending_cost + next_cost
            )
    return states[len(string)][tuple()]


def main():
    n = int(input())
    strings = [input() for _ in range(n)]
    for string in strings:
        print(stack(string))


if __name__ == "__main__":
    # print(stack("abba"))
    # print(stack("rollover ahead"))
    # print(stack("ogopogo spotted!"))
    main()
