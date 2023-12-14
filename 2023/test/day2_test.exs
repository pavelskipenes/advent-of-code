defmodule Day2Test do
  use ExUnit.Case

  test "part 1 test" do
    assert 8 == AdventOfCode.get_puzzle_test(2,1)
    |> Day2.Part1.solve
  end

  # test "part 1" do
  #   AdventOfCode.get_puzzle(2)
  #   |> Part1.solve
  #   |> IO.inspect
  # end
  #
  # test "part 2 test" do
  #   assert VAL == AdventOfCode.get_puzzle_test(2, 2)
  #   |> Part2.solve
  # end
  #
  # test "part 2" do
  #   AdventOfCode.get_puzzle(2)
  #   |> Part2.solve
  #   |> IO.inspect
  # end
end
