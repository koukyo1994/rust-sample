def sort(x: list, up: bool = True):
    if len(x) <= 1:
        return x
    else:
        mid_point = len(x) // 2

        first = sort(x[:mid_point], up=True)
        second = sort(x[mid_point:], up=False)

        x1 = first + second
        return _sub_sort(x1, up)


def _sub_sort(x: list, up: bool = True):
    if len(x) == 1:
        return x

    else:
        _compare_and_swap(x, up)

        mid_point = len(x) // 2

        first = _sub_sort(x[:mid_point], up)
        second = _sub_sort(x[mid_point:], up)

        return first + second


def _compare_and_swap(x, up):
    mid_point = len(x) // 2
    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == up:
            x[i], x[mid_point + i] = x[mid_point + i], x[i]


if __name__ == '__main__':
    nums = [10, 30, 11, 20, 4, 330, 21, 110]
    print(sort(nums, up=True))
    print(sort(nums, up=False))
