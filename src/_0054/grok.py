# correct_winners.py
from collections import Counter

value_map = {
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    "T": 10,
    "J": 11,
    "Q": 12,
    "K": 13,
    "A": 14,
}


def parse_hand(cards):
    values = [value_map[c[0]] for c in cards]
    suits = [c[1] for c in cards]
    value_counts = Counter(values)
    suit_counts = Counter(suits)
    sorted_values = sorted(values, reverse=True)
    return values, suits, value_counts, suit_counts, sorted_values


def is_flush(suit_counts):
    return max(suit_counts.values()) == 5


def is_straight(values, value_counts):
    if len(value_counts) != 5:
        return False, 0
    unique = sorted(value_counts.keys())
    if unique == [2, 3, 4, 5, 14]:
        return True, 5
    if unique[4] - unique[0] == 4:
        return True, unique[4]
    return False, 0


def hand_rank(cards):
    values, suits, value_counts, suit_counts, sorted_values = parse_hand(cards)
    flush = is_flush(suit_counts)
    straight, straight_high = is_straight(values, value_counts)

    fours = [v for v, c in value_counts.items() if c == 4]
    threes = [v for v, c in value_counts.items() if c == 3]
    pairs = [v for v, c in value_counts.items() if c == 2]
    fours.sort(reverse=True)
    threes.sort(reverse=True)
    pairs.sort(reverse=True)

    if flush and straight:
        return (9, straight_high, sorted_values)
    if fours:
        return (8, fours[0], sorted_values)
    if threes and pairs:
        return (7, threes[0], pairs[0], sorted_values)
    if flush:
        return (6, sorted_values)
    if straight:
        return (5, straight_high, sorted_values)
    if threes:
        return (4, threes[0], sorted_values)
    if len(pairs) == 2:
        return (3, pairs[0], pairs[1], sorted_values)
    if pairs:
        return (2, pairs[0], sorted_values)
    return (1, sorted_values)


def main():
    with open("poker.txt", "r") as f:
        lines = f.readlines()

    with open("correct_winners.txt", "w") as out:
        player1_wins = 0
        for line in lines:
            cards = line.strip().split()
            if len(cards) != 10:
                continue
            hand1 = cards[:5]
            hand2 = cards[5:]

            rank1 = hand_rank(hand1)
            rank2 = hand_rank(hand2)

            if rank1 > rank2:
                out.write("1\n")
                player1_wins += 1
            else:
                out.write("2\n")

        print(
            f"Generated correct_winners.txt — Player 1 wins {player1_wins} hands (should be 376)"
        )


if __name__ == "__main__":
    main()
