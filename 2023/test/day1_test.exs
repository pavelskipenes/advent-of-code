defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "test puzzle one" do
    {result, file_content} = File.read("puzzle_input/day_1_test.txt")
    assert result == :ok
    assert 142 == Day1.solver_part1(file_content)
  end

  test "problem 1" do
    assert 55208 == File.read!("puzzle_input/day_1.txt")
    |> Day1.solver_part1
  end

end
