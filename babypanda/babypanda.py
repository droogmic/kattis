import logging

logging.basicConfig(level="INFO")

def sneezes(number_of_days: int, number_of_slimes: int):
    """
    m ~= 2^n
    let's try calculating all possibilities backwards.
    """
    logging.info("%s, %s", number_of_days, number_of_slimes)

    if number_of_slimes == 0:
        return 0
    if number_of_days == 0:
        return None

    previous_day_idx = number_of_days - 1
    
    if number_of_slimes % 2 != 0:
        return None
    
    previous_count_without_sneeze = number_of_slimes // 2
    without_sneeze = sneezes(previous_day_idx, previous_count_without_sneeze)
    if without_sneeze is not None:
        return without_sneeze
    
    previous_count_with_sneeze = previous_count_without_sneeze - 1
    if previous_count_with_sneeze < 0:
        return None

    return 1 + sneezes(previous_day_idx, previous_count_with_sneeze)

def main() -> None:
    n, m = (int(v) for v in input().split())
    sneeze_count = sneezes(n, m)
    print(sneeze_count)

if __name__ == "__main__":
    main()