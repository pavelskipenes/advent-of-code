defmodule AdventOfCode do
  def get_puzzle(day) when is_integer(day) do
      "puzzle_input/day_#{day}/input.txt"
      |> File.read!
  end

  def get_puzzle_test(day, part) when is_integer(day) and is_integer(part) do
      "puzzle_input/day_#{day}/part#{part}.txt"
      |> File.read!
  end
end
