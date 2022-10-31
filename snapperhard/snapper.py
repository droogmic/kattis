for _ in range(int(input())):
    N, K = (int(v) for v in input().split())
    mask = (1 << N) - 1
    result = (mask & K) ^ mask
    print("Case #1: {}".format("OFF" if result else "ON"))
