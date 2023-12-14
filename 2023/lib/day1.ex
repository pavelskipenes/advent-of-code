defmodule Day1 do
  use ExUnit.Case

  def first_and_last(list) when is_list(list) do
    [List.first(list), List.last(list)]
  end

  # convert a list as if were one integer. First index is the most significant digit.
  def to_integer(list) when is_list(list) do
    list
    |> Enum.reverse()
    |> to_integer(0)
  end

  defp to_integer(list, power) when is_list(list) and is_integer(power) do
    case list do
      [] -> 0
      [head | tail] when is_integer(head) -> head * :math.pow(10, power) + to_integer(tail, power + 1)
    end
  end

  defmodule Part1 do
    def solve(input) when is_binary(input) do
      input
      |> String.trim
      |> String.split("\n")
      |> Enum.map(&line_solver/1)
      |> Enum.sum
    end

    defp line_solver(line) when is_binary(line) do
      line
      |> String.to_charlist() # list of unicode numbers
      |> Enum.filter(fn char -> char in ?1..?9 end) # filter out numerical ascii values
      |> Day1.first_and_last
      |> Enum.map(fn el -> el - 0x30 end) # ascii number to number
      |> Day1.to_integer
    end
  end

  defmodule Part2 do
    def solve(input) when is_binary(input) do
      input
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(&line_solver/1)
      |> Enum.sum
    end

    def line_solver(line) do
      line
      |> String.to_charlist
      #|> extract_numbers
      |> extract_numbers_shared
      |> Day1.first_and_last
      |> Day1.to_integer
    end

    # Extract numbers from a string. Characters cannot be shared
    def extract_numbers(characters) when is_list(characters) do
      case characters do
        []                                        -> []
        [number    | rest] when number in ?1..?9  -> [number - 0x30 | extract_numbers(rest)]
        ~c"one"   ++ rest                         -> [1 | extract_numbers(rest)]
        ~c"two"   ++ rest                         -> [2 | extract_numbers(rest)]
        ~c"three" ++ rest                         -> [3 | extract_numbers(rest)]
        ~c"four"  ++ rest                         -> [4 | extract_numbers(rest)]
        ~c"five"  ++ rest                         -> [5 | extract_numbers(rest)]
        ~c"six"   ++ rest                         -> [6 | extract_numbers(rest)]
        ~c"seven" ++ rest                         -> [7 | extract_numbers(rest)]
        ~c"eight" ++ rest                         -> [8 | extract_numbers(rest)]
        ~c"nine"  ++ rest                         -> [9 | extract_numbers(rest)]
        [_         | rest]                        -> extract_numbers(rest)
      end
    end

    # Extract numbers from a string. Characters can be shared
    def extract_numbers_shared(characters) when is_list(characters) do
      case characters do
        []                                        -> []
        [number    | rest] when number in ?1..?9  -> [number - 0x30 | extract_numbers_shared(rest)]
        ~c"one"   ++ rest                         -> [1 | extract_numbers_shared(~c"ne" ++ rest)]
        ~c"two"   ++ rest                         -> [2 | extract_numbers_shared(~c"wo" ++ rest)]
        ~c"three" ++ rest                         -> [3 | extract_numbers_shared(~c"hree" ++ rest)]
        ~c"four"  ++ rest                         -> [4 | extract_numbers_shared(~c"our" ++ rest)]
        ~c"five"  ++ rest                         -> [5 | extract_numbers_shared(~c"ive" ++ rest)]
        ~c"six"   ++ rest                         -> [6 | extract_numbers_shared(~c"ix" ++ rest)]
        ~c"seven" ++ rest                         -> [7 | extract_numbers_shared(~c"even" ++ rest)]
        ~c"eight" ++ rest                         -> [8 | extract_numbers_shared(~c"ight" ++ rest)]
        ~c"nine"  ++ rest                         -> [9 | extract_numbers_shared(~c"ine" ++ rest)]
        [_ | rest]                                -> extract_numbers_shared(rest)
      end
    end

  end

end
