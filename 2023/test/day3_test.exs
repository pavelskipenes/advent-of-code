defmodule Day3Test do
  use ExUnit.Case

  test "parse shit" do
    {symbols, numbers} = AdventOfCode.get_puzzle_test(3, 1)
      |> Day3.Part1.parse_shit

    assert symbols |> Enum.reverse == [{1, 3}, {3, 6}, {4, 3}, {5, 5}, {8, 3}, {8, 5}]

    assert numbers |> Enum.reverse == [
      {{0, 0}, 467},
      {{0, 5}, 114},
      {{2, 2}, 35},
      {{2, 6}, 633},
      {{4, 0}, 617},
      {{5, 7}, 58},
      {{6, 2}, 592},
      {{7, 6}, 755},
      {{9, 1}, 664},
      {{9, 5}, 598}
    ]
  end

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
