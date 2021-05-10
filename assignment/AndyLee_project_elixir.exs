hand1 = [3, "H", 10, "S", 4, "S", 4, "C", 5, "C"]
hand2 = [2, "H", 2, "S", 5, "S", 2, "C", 13, "C"]

ace_straight = [2, "H", 3, "S", 14, "S", 4, "C", 5, "C"]
high_card = [10, "H", 2, "S", 14, "S", 4, "D", 5, "C"]
pair = [2, "H", 2, "S", 14, "S", 4, "C", 5, "C"]
high_pair = [14, "H", 2, "S", 14, "S", 4, "C", 5, "C"]
two_pair = [2, "H", 2, "S", 14, "S", 14, "C", 5, "C"]
two_pair2 = [14, "H", 14, "S", 13, "D", 13, "C", 5, "C"]
straight = [10, "D", 13, "S", 12, "S", 11, "D", 14, "H"]
straight2 = [10, "S", 13, "D", 12, "C", 11, "H", 14, "S"]
three_kind = [14, "H", 14, "S", 14, "D", 13, "C", 5, "C"]
full_house = [14, "H", 14, "S", 14, "D", 5, "C", 5, "C"]
flush = [5, "C", 7, "C", 9, "C", 11, "C", 12, "C"]
flush2 = [5, "S", 7, "S", 9, "S", 11, "S", 13, "S"]
straight_flush = [5, "S", 7, "S", 9, "S", 6, "S", 8, "S"]
straight_flush2 = [14, "D", 2, "D", 3, "D", 5, "D", 4, "D"]
royal_flush = [14, "S", 13, "S", 11, "S", 12, "S", 10, "S"]

IO.puts "\n"

Poker.winner hand1,hand2
Poker.winner two_pair2,high_card
Poker.winner ace_straight,hand2
Poker.winner straight,straight_flush
Poker.winner straight,straight2
Poker.winner two_pair2,two_pair
Poker.winner two_pair2,high_pair
Poker.winner pair,high_pair
Poker.winner three_kind,full_house
Poker.winner flush,flush2
Poker.winner straight_flush,straight_flush2
Poker.winner straight_flush2,flush2
Poker.winner straight_flush2,royal_flush
Poker.winner two_pair2,royal_flush


IO.puts "\n"

# DEBUGGING
# IO.inspect Poker.calculate_score(straight)
# IO.inspect Poker.calculate_score(straight2)