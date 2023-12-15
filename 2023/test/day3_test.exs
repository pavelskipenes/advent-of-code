defmodule Day3Test do
  use ExUnit.Case

  test "part 1 test" do
    assert 4361 == AdventOfCode.get_puzzle_test(3, 1)
    |> Day3.Part1.solve
    |> IO.inspect
  end

  # test "part 1" do
  #   AdventOfCode.get_puzzle(3)
  #   |> Day3.Part1.solve
  #   |> IO.inspect
  # end

  # test "part 2 test" do
  #   AdventOfCode.get_puzzle_test(3, 2)
  #   |> Day3.Part2.solve
  #   |> IO.inspect
  # end

  # test "part 2" do
  #   AdventOfCode.get_puzzle(3)
  #   |> Day3.Part2.solve
  #   |> IO.inspect
  # end
end
