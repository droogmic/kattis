import logging
from dataclasses import dataclass, field
from itertools import combinations
from typing import (
    Annotated,
    FrozenSet,
    List,
    NewType,
    Set,
    Tuple,
    Union,
    Dict,
    Optional,
)

Computer = NewType("Computer", int)
Connection = Annotated[FrozenSet[Computer], 2]


@dataclass
class Group:
    """
    >>> g = Group()
    >>> g.add_connection([1, 2])
    >>> g
    Group(members={1, 2}, connections_list=[frozenset({1, 2})], connection_map={1: {2}, 2: {1}})
    >>> 1 in g
    True
    >>> [0, 1] in g
    True
    >>> frozenset([1, 0]) in g
    True
    >>> [0, 3] in g
    False
    """

    members: Set[Computer] = field(default_factory=set)
    connections_list: List[Connection] = field(default_factory=list)
    connection_map: Dict[Computer, Set[Computer]] = field(default_factory=dict)

    def __str__(self):
        return f"{[set(c) for c in self.connections_list]}"

    def __contains__(self, val: Union[Computer, Connection]) -> bool:
        if isinstance(val, int):
            return val in self.members
        return any(v in self.members for v in val)

    def add_connection(self, connection: Connection):
        logging.debug(f"add_{connection=}")
        self.members.update(connection)
        self.connections_list.append(frozenset(connection))
        first, second = connection
        self.connection_map.setdefault(first, set()).add(second)
        self.connection_map.setdefault(second, set()).add(first)

    def longest_path(self) -> int:
        pairs: Dict[Connection, Optional[int]] = {
            frozenset(pair): None for pair in (combinations(self.members, 2))
        }
        logging.debug(f"{pairs=}")
        for connection in self.connections_list:
            pairs[connection] = 1
        logging.debug(f"{pairs=}")
        while None in pairs.values():
            for pair, dist in pairs.items():
                first, second = pair
                if dist is not None:
                    continue
                possible_routes = [
                    pairs.get(frozenset([next_step, second]))
                    for next_step in self.connection_map[first]
                ]
                possible_routes = [dist for dist in possible_routes if dist is not None]
                if not possible_routes:
                    continue
                best = min(possible_routes)
                pairs[pair] = best + 1
                logging.debug(f"{pairs=}")
        return max(pairs.values())  # type: ignore


def get_groups(connections) -> List[Group]:
    """
    >>> groups = get_groups([
    ...     (0, 1),
    ...     (1, 2),
    ...     (2, 3),
    ...     (3, 4),
    ... ])
    >>> len(groups)
    1
    >>> groups = get_groups([
    ...     (0, 1),
    ...     (1, 2),
    ...     (3, 4),
    ...     (5, 6),
    ...     (6, 7),
    ...     (1, 8),
    ... ])
    >>> len(groups)
    3
    """
    logging.debug(f"get_groups({connections=})")
    groups: List[Group] = []
    while connections:
        group = Group()
        group.add_connection(connections.pop())
        while True:
            for connection in connections:
                if connection in group:
                    group.add_connection(connection)
                    break
            else:
                break  # skip removal
            connections.remove(connection)

        groups.append(group)
        logging.debug(f"{groups=}")
    return groups


def get_max_hops(connections) -> int:
    groups = get_groups(connections)
    logging.debug(f"{groups=}")
    longest_paths = sorted(group.longest_path() for group in groups)
    logging.debug(f"{longest_paths=}")
    if len(longest_paths) == 0:
        return 0
    worst = longest_paths.pop()
    if len(longest_paths) == 0:
        return (worst + 1) // 2
    second_worst = longest_paths.pop()
    return ((worst + 1) // 2) + ((second_worst + 1) // 2) + 1


def main():
    computers, cables = (int(v) for v in input().split())
    connections = [
        frozenset(Computer(v) for v in input().split()) for _ in range(cables)
    ]
    logging.debug(f"{connections=}")
    max_hops = get_max_hops(connections)
    print(max_hops)


if __name__ == "__main__":
    import doctest

    logging.basicConfig(level="ERROR")
    doctest.testmod()
    logging.basicConfig(level="DEBUG")
    main()
