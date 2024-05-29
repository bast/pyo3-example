class: gray-background, middle, center

# How I write Rust and make it look like Python

**Radovan Bast**, UiT The Arctic University of Norway

Nordic RSE conference 2024

<img src="img/logo.png"
     alt="RSE logo"
     style="height: 250px;"/>

---

class: center, middle

.quote["There are only two kinds of languages: the ones people complain about and the ones nobody uses."]

Bjarne Stroustrup

---

.left-column50[
# Python

- Popular

- Great for prototyping

- Easy to find help

- Many people are familiar with `pip install` and `conda`

- .emph[Library ecosystem]
]

--

.right-column50[
# Rust

- Fast

- Memory safety

- Thread safety

- Type safety

- .emph[Great tooling]
]

---

# 6 is a perfect number

## Proper divisors of 6 are 1, 2, 3

## 1 + 2 + 3 = 6

--

# 28 is a perfect number

## Proper divisors of 28 are 1, 2, 4, 7, 14

## 1 + 2 + 4 + 7 + 14 = 28

---

```python
from tqdm import tqdm


def proper_divisors(n: int) -> list[int]:
    return [x for x in range(1, n) if n % x == 0]


def number_is_perfect(n: int) -> bool:
    return sum(proper_divisors(n)) == n


n = 10000
numbers = [i for i in tqdm(range(1, n)) if number_is_perfect(i)]

print(numbers)
```

- First four perfect numbers are **6, 28, 496, 8128**. Already mentioned in Euclid's Elements (circa 300 BC).

- It took 1500 years to find the next perfect number .cite[Shams ad-Dīn Abû’t-Tāhir Ismāʽīl ibn-Ibrāhīm ibn-Ġāzī ibn-ʽAlī ibn Muhammad al-Ḥanafī al-Māridīnī (1194-1252)].

- .emph[We will try to find the next with the help of Python and Rust.]

- Unsolved problem: Are there any odd perfect numbers?

---

# Demo time

- https://github.com/bast/pyo3-example

- **This was just a test**: In this case deployed to
  [test-PyPI](https://test.pypi.org/project/perfectseries/).

- **Advantage**: You don't need any Rust to run it. Looks like Python, behaves
  like Python.

- **Downside**: Once Python releases 3.13, we will need to update the GitHub
  Actions workflow and release a new version of the package.
