defmodule Day1 do
  use ExUnit.Case

  def solver_part1(input) when is_binary(input) do
    String.split(input, "\n")
    |> Enum.map(&convert_number/1)
    |> Enum.reduce(0, fn (element, accumulator) -> accumulator + element end)
  end

  defp convert_number(line) when is_binary(line) do
    line
    |> String.to_charlist
    |> Enum.filter(&is_ascii_number/1)
    |> first_and_last 
    |> List.to_string
    |> String.to_integer
  end

  defp is_ascii_number(char) when char in ?0..?9 do
    true
  end 

  defp is_ascii_number(_char) do
    false
  end 

  defp first_and_last(list) when is_list(list) and length(list) >= 1 do
    [List.first(list) , List.last(list)]
  end

  defp first_and_last(list) when is_list(list) and length(list) == 0 do
    [?0]
  end
end
