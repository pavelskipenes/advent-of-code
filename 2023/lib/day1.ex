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

    # this solver walkes the line and will either discard one character at a time or multiple if they match an expected value. If the input has overlapping values like "oneight" then it will only find "one" and the remainder will be "ight" which will not match "eight"
    def line_solver(line) do
      line
      |> String.graphemes()
      |> extract_numbers
      |> Day1.first_and_last
      |> Enum.map(fn string -> Integer.parse(string) end)
      |> Enum.map(fn result -> {value, _} = result; value end)
      |> Day1.to_integer
    end


    def string_to_number(string) when is_binary(string) do
      case string do
        "one" -> {:ok, 1}
        "two" -> {:ok, 2}
        "three" -> {:ok, 3}
        "four" -> {:ok, 4}
        "five" -> {:ok, 5}
        "six" -> {:ok, 6}
        "seven" -> {:ok, 7}
        "eight" -> {:ok, 8}
        "nine" -> {:ok, 9}
        "1" -> {:ok, 1}
        "2" -> {:ok, 2}
        "3" -> {:ok, 3}
        "4" -> {:ok, 4}
        "5" -> {:ok, 5}
        "6" -> {:ok, 8}
        "7" -> {:ok, 7}
        "8" -> {:ok, 8}
        "9" -> {:ok, 9}
        _ -> {:error, "no match"}
      end
    end

    def convert_number(line) when is_binary(line) do
      line
      |> possible_string_permutations(3..5)
      |> Enum.map(&string_to_number/1)
      |> Enum.filter(fn
        {:ok, _} -> true
        _ -> false
      end)
      |> Enum.map(fn {:ok, val} -> val end)
      |> Day1.to_integer
    end


    def possible_string_permutations(line, window_size) when is_binary(line) do
      window_size
      |> Enum.map(fn window_size -> windows(String.to_charlist(line), window_size) end)
      |> Enum.map(&string_to_number/1)
      |> List.flatten()
    end

    def valid_permutations(string_permutations) do
      string_permutations
      |> Enum.map(fn potential_number -> string_to_number(potential_number) end)
      |> Enum.filter(fn
        {:ok, _} -> true
        _ -> false
      end)
      |> Enum.map(fn {_, value} -> value end)
    end

    def windows(list, size) do
      windows_slice_indexies(list, size)
      |> Enum.map(fn starting_index -> Enum.slice(list, starting_index, size) end)
    end

    def windows_slice_indexies(list, size)
      when is_list(list) and size >= 1 and length(list) >= size do
      0..(length(list) - size)
    end

    def strings_to_number(list_of_strings) when is_list(list_of_strings) do
      list_of_strings
      |> Enum.map(&string_to_number/1)
    end

    def extract_numbers(graphemes) when is_list(graphemes) do
      case graphemes do
        [] -> []
        [number | rest] when number in ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"] -> [number | extract_numbers(rest)]
        ["o", "n", "e" | rest] ->
          ["1" | extract_numbers(rest)]
        ["t", "w", "o" | rest] ->
          ["2" | extract_numbers(rest)]
        ["t", "h", "r", "e", "e" | rest] ->
          ["3" | extract_numbers(rest)]
        ["f", "o", "u", "r" | rest] ->
          ["4" | extract_numbers(rest)]
        ["f", "i", "v", "e" | rest] ->
          ["5" | extract_numbers(rest)]
        ["s", "i", "x" | rest] ->
          ["6" | extract_numbers(rest)]
        ["s", "e", "v", "e", "n" | rest] ->
          ["7" | extract_numbers(rest)]
        ["e", "i", "g", "h", "t" | rest] ->
          ["8" | extract_numbers(rest)]
        ["n", "i", "n", "e" | rest] ->
          ["9" | extract_numbers(rest)]
        [_ | rest] ->
          extract_numbers(rest)
      end
    end

  end

end
