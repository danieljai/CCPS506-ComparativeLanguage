defmodule Lab2_1 do
    def powers(x) when not is_number(x), do: :error
    def powers(x), do: :math.pow(x,2)

    def powers(x,y) when not is_number(x) or not is_number(y) or (x < 0 and not is_integer(y)) , do: :error
    def powers(x,y), do: :math.pow(x,y)
    def powers(), do: 0
    
end

defmodule Lab2_2 do
    def all(x) do
        cond do
            not is_list(x) ->
                :err_not_list
            Enum.any?(x,fn(n)-> not is_integer(n) end) ->
                :err_invalid_value
            True ->
                :success
                [h | rest] = x
                add_myself(rest, h)
        end               
    end

    def even(x) do
        cond do
            not is_list(x) ->
                :err_not_list
            Enum.any?(x,fn(n)-> not is_integer(n) end) ->
                :err_invalid_value
            Enum.any?(x,fn(n)-> :math.fmod(n,2)>0 end) ->
                :err_odd_integer_detected
            True ->
                :success
                [h | rest] = x
                add_myself(rest, h)
        end               
    end

    def odd(x) do
        cond do
            not is_list(x) ->
                :err_not_list
            Enum.any?(x,fn(n)-> not is_integer(n) end) ->
                :err_invalid_value
            Enum.any?(x,fn(n)-> not (:math.fmod(n,2)>0) end) ->
                :err_even_integer_detected
            True ->
                :success
                [h | rest] = x
                add_myself(rest, h)
        end               
    end

    def add_myself(x,y) do
        if (length x) > 0 do
            [h | rest] = x
            add_myself(rest, y+h)
        else
            y
        end
    end

    
end

defmodule Lab2_3 do
    def enum_lab3(x, y) do        
        cond do
            not is_list(x) ->
                :err_not_list
            (length x) == 0 ->
                x
            True ->
                :success
                [h | rest] = x
                if is_number(h) do
                    [y.(h)|enum_lab3(rest,y)]
                else
                    [(h)|enum_lab3(rest,y)]
                end
        end
    end
end