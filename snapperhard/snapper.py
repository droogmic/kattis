for idx in range(1, int(input()) + 1):
    N, K = (int(v) for v in input().split())
    mask = (1 << N) - 1
    result = (mask & K) ^ mask
    print("Case #{}: {}".format(idx, "OFF" if result else "ON"))
