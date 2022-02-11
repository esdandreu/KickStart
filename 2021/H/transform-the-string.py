def solution(S: str, F: str) -> int:
    sol = 0
    base = ord('a') - 1
    limit = ord('z') - base
    for char in S:
        v = ord(char) - base
        possibilities = []
        for option in F:
            option = ord(option) - base
            possibilities.append(abs(v - option))
            possibilities.append(limit - abs(v - option))
        sol += min(possibilities)
    return sol


if __name__ == '__main__':
    t = int(input())
    for i in range(1, t + 1):
        S = input()
        F = input()
        print(f"Case #{i}: {solution(S, F)}")