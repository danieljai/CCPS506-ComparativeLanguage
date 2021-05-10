# Andy Lee - 500163559
# Elixir

defmodule Poker do
    def cards_to_nums(x) do
        # convert cards to numerical representations of 0 to 51
        cond do
            not is_list(x) ->
                :err_not_list
            (length x) == 0 ->
                x
            True ->

                :success
                [current_rank | rest] = x
                [current_suit | rest] = rest
                modifier =
                    case current_suit do
                        "D" -> 0*13
                        "C" -> 1*13
                        "H" -> 2*13
                        "S" ->  3*13
                    end
                [(modifier+current_rank-2)|cards_to_nums(rest)]
        end

    end

    # This function is NOT USED
    def nums_to_cards(x) do
        # convert numerials back to representable cards
        cond do
            not is_list(x) ->
                :err_not_list
            (length x) == 0 ->
                #x
                []
            True ->

                :success
                [current_num | rest] = x
                current_rank = rem(current_num,13)+2
                current_suit =
                    case div(current_num,13) do
                        0 -> "D"
                        1 -> "C"
                        2 -> "H"
                        3 -> "S"
                    end
                [current_rank,current_suit]++nums_to_cards(rest)
        end

    end

    def find_same_kinds(hand, kind) do
        [first_num|rest] = hand
        # IO.inspect hand, label: "inspecting hand"
        # IO.inspect kind, label: "kinds"
        cond do
            not is_list(hand) ->
                # not needed, but kept it here anyways
                :err_not_list
            (length hand) < kind ->
                # if the number of search kind > length of the hand, then stop
                {0}
            True ->
                if List.starts_with?(hand, List.duplicate(first_num, kind)) do
                    :true
                    # IO.puts "found match: "
                    #IO.inspect first_num, label: "found match"
                    # return tuple result
                    {kind, first_num,hand--List.duplicate(first_num,kind)}
                else
                    :false
                    # recursion, keep searching
                    find_same_kinds(rest,kind)
                end
        end
    end

    def calculate_score(hand) do
        # normalizing hand into numbers for easy computating
        # represent hand in ranks
        card_rank = Cards.isolate_rank(Cards.cards_to_nums(hand))
        # represent hand in suits
        card_suit = Cards.isolate_suit(Cards.cards_to_nums(hand))
        # minus all ranks by the smallest rank in a hand
        reduced_hand = Enum.map(card_rank,fn(n)->n-(hd card_rank) end)

        # DEBUG
        # IO.inspect hand, label: "ORIGINAL"
        # IO.inspect Cards.cards_to_nums(hand), label: "numerical"
        # IO.inspect card_rank, label: "rank"
        # IO.inspect card_suit, label: "suit"
        # DEBUG END

        cond do
            Enum.all?(card_suit,fn(n)->n==(hd card_suit) end) ->
                :all_suits

                cond do
                    List.starts_with?(card_rank,[8,9,10,11,12]) ->
                        :royal_flush
                        # score BASE 10 + suit
                        # 10 J Q K A
                        # possibilities 4
                        (hd card_suit) + 10000000

                    List.starts_with?(reduced_hand,[0,1,2,3,12]) ->
                        :ace_lead_straight_flush
                        # score: BASE 9 + suit
                        # A 2 3 4 5
                        # possibilities + 4
                        (hd card_suit) + 9000000

                    List.starts_with?(reduced_hand,[0,1,2,3,4]) ->
                        :non_ace_lead_straight_flush
                        # score: BASE 9.5 + highest card (6 diamonds ~ K spades)
                        # (2 3 4 5 6 ~ 9 10 J Q K) * 4
                        # posssibilities 32
                        #(tl card_rank)*10 + 9500000
                        high_card(hand) + 9500000
                    true ->
                        :flush
                        # score: BASE 6 +  highest card rank
                        # any 5 rank card with same suit
                        # poss. 5,108                        
                        (List.last(card_rank))*10 + 6000000

                end

            elem(Cards.find_same_kinds(card_rank,4),0) == 4 ->
                :four_of_a_kind
                # score: BASE 8 + 4-of-a-kind card rank
                # 3 3 3 3 13
                # poss. 13 ~ relistic 624
                elem(Cards.find_same_kinds(card_rank,4),1) + 8000000

            elem(Cards.find_same_kinds(card_rank,3),0) == 3 ->
                :found_triples

                triple_card = elem(Cards.find_same_kinds(card_rank,3),1)
                triple_list = List.duplicate(triple_card,3)
                rest = card_rank--triple_list
                if List.first(rest) == List.last(rest) do
                    # full house
                    # score: BASE 7 + rank card with triple
                    # poss. 13 (we only care its a pair, don't care what it is) ~ relistic 3,744
                    triple_card + 7000000
                else
                    # 3 of a kind
                    # score: BASE 4 + rank card with triple
                    # poss. 13 (we only care it isn't a pair, don't care what it is) ~ relistic 54,912
                    triple_card + 4000000
                end

            elem(Cards.find_same_kinds(card_rank,2),0) == 2 ->                
                :found_pair
                # get rank of first pair
                first_pair = elem(Cards.find_same_kinds(card_rank,2),1)
                # create new list of pair with first_pair
                first_pair_list = List.duplicate(first_pair,2)
                # remove 2 occurrence of the first_pair from hand
                rest = card_rank--first_pair_list
                # attempts to find second pair
                second_pair_tuple = Cards.find_same_kinds(rest,2)
                                
                # if second_pair_tuple returns a 2 at index 0, second pair is found
                if elem(second_pair_tuple,0) == 2 do
                    # two pairs
                    # because list is sorted before finding pairs, first pair < second pair
                    # score: BASE 3
                    # realistic 123,552

                    # get rank of second pair
                    second_pair = elem(second_pair_tuple,1)

                    first_pair*10 + high_suit(hand,first_pair) + (second_pair*10 + high_suit(hand,second_pair) )*1000 + 3000000
                else
                    # one pair
                    first_pair*10 + high_suit(hand,first_pair) + 2000000
                end

            List.starts_with?(reduced_hand,[0,1,2,3,12]) ->
                :straight_ace_lead
                # straight starting with Ace lead, the lowest 4 straight hands
                # A 2 3 4 5
                # score: BASE 5 + high card
                # poss. 4
                high_card(hand) + 5000000
            List.starts_with?(reduced_hand,[0,1,2,3,4]) ->
                :straight_non_ace_lead
                # all other non-ace-leading straight 
                # 2 3 4 5 6 ~ 10 J Q K A
                # score: BASE 5.1 + high card 
                high_card(hand) + 5100000
            true ->
                # high card
                # score: BASE 0

                high_card(hand)                
        end
    end

    def high_card(hand) do
        # return numerical value of highest card in hand
        round((List.last(Enum.sort(Enum.map(Cards.cards_to_nums(hand),fn(x)-> rem(x,13) + (0.1*div(x,13))end))))*10)
    end

    def high_suit(hand,rank) do
        # given a hand, find the highest suit for a specified rank
        num_lookup = [0+rank, 13+rank, 26+rank, 39+rank]

        # poorman's list UNION
        # extracting possible cards calculated from given 'rank'
        org_list = Cards.cards_to_nums(hand)
        exclude_list = org_list -- num_lookup
        result_list = org_list -- exclude_list

        # DEBUG STARTS
        # IO.puts "... computing pairs"
        # IO.inspect org_list, label: "GIVEN"
        # IO.inspect num_lookup, label: "lookup"
        # IO.inspect result_list, label: "result_list"
        # DEBUG ENDS
        

        # returns highest suit
        List.last(isolate_suit(result_list))
    end

    def isolate_rank(hand) do
        # isolate only hands' rank
        Enum.sort(Enum.map(hand,fn(x)->rem(x,13) end))
    end

    def isolate_suit(hand) do
        # isolate only hands' suit
        Enum.sort(Enum.map(hand,fn(x)->div(x,13) end))
    end

    def winner(hand1, hand2) do
        if calculate_score(hand1) > calculate_score(hand2) do
            IO.inspect hand1, label: "Winner hand 1"
            IO.inspect hand2, label: "Losing hand 2"
        else
            IO.inspect hand2, label: "Winner hand 2"
            IO.inspect hand1, label: "Losing hand 1"
        end
        IO.puts "----------------------------------------------------"
    end
end
