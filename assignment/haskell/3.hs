-- Andy Lee - 500163559
-- Haskell

-- Represent each hand as a list of pair tuples. Name your function for finding the winning hand "w"nner”, and call it as follows:
-- hand1 = [(3, "H"), (10, "S"), (4, "S"), (4, "C"), (5, "C")]
-- hand2 = [(2, "H"), (2, "S"), (5, "S"), (2, "C"), (13, "C")]
-- winner hand1 hand2

-- Your winner function and all associated helper functions should be defined in a module called Poker, in a file Poker.hs. I should be able to load the Poker module into GHCi and call your winner function.
module Poker where
    data Suit = S | H | C | D
        deriving (Read, Show, Enum, Eq, Ord)

    data Rank = 1..10

    data Card rank suit = Card (rank, suit)
        deriving (Show)