colormap = {
    'U'  : set(),
    'R'  : {'R'},
    'Y'  : {'Y'},
    'B'  : {'B'},
    'O'  : {'R', 'Y'},
    'P'  : {'R', 'B'},
    'G'  : {'Y', 'B'},
    'A'  : {'R', 'Y', 'B'},
}
def solution(P: str):
    painting = [colormap[square] for square in P]
    strokes = 0
    stroke = set()
    for colors in painting:
        strokes += len(colors - stroke)
        stroke = colors
    return strokes

if __name__ == '__main__':
    t = int(input())
    for i in range(1, t + 1):
        N = input()
        P = str(input())
        print(f"Case #{i}: {solution(P)}")