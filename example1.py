from tqdm import tqdm


def proper_divisors(n: int) -> list[int]:
    return [x for x in range(1, n) if n % x == 0]


def number_is_perfect(n: int) -> bool:
    return sum(proper_divisors(n)) == n


n = 10000
numbers = [i for i in tqdm(range(1, n)) if number_is_perfect(i)]

print(numbers)
