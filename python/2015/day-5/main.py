from collections import defaultdict

def not_contains_forbidden_string(str_slice) -> bool:
    forbidden = ("ab", "cd", "pq", "xy")
    return True if str_slice not in forbidden else False

def vowel_counts(char: str, vowels: dict):
    if char in ("a", "e", "i", "o", "u"):
        vowels[char] += 1
    return vowels


def main(input_string: str) -> bool:
    """
    Forbidden letters -> must check entire string
    Vowels -> once three detected stop
    Duplicated -> once detected stop
    """
    slice_len = len(input_string)
    index = 1

    has_vowels = False
    has_double = False
    no_forbidden = True

    vowels = defaultdict(int)
    vowels =vowel_counts(input_string[0], vowels)

    while index < slice_len:

        first_char, second_char = input_string[index-1], input_string[index]
        if not has_double:
            has_double = first_char == second_char

        if not has_vowels:
            vowels = vowel_counts(second_char, vowels)
            has_vowels = sum(vowels.values()) == 3

        if no_forbidden:
            no_forbidden = not_contains_forbidden_string(first_char + second_char)

        is_nice_string = (has_vowels, has_double, no_forbidden)
        index += 1

    return all(is_nice_string)

if __name__ == "__main__":

    result = main("ugknbfddgicrmopn")
    assert(result), f"Failed: ugknbfddgicrmopn, Expect True, Result: {result}"

    result = main("aaa")
    assert(result), f"Failed: aaa, Expect True, Result: {result}"

    result = main("jchzalrnumimnmhp")
    assert(not result), f"Failed: jchzalrnumimnmhp, Expect False, Result: {result}"

    result = main("haegwjzuvuyypxyu")
    assert(not result), f"Failed: haegwjzuvuyypxyu, Expect False, Result: {result}"

    result = main("dvszwmarrgswjxmb")
    assert(not result), f"Failed: dvszwmarrgswjxmb, Expect False, Result: {result}"
