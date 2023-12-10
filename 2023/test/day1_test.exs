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

  test "number matching" do
    assert Day1.string_to_number("one") == 1
    assert Day1.string_to_number("two") == 2
    assert Day1.string_to_number("three") == 3
    assert Day1.string_to_number("four") == 4
    assert Day1.string_to_number("five") == 5
    assert Day1.string_to_number("six") == 6
    assert Day1.string_to_number("seven") == 7
    assert Day1.string_to_number("eight") == 8
    assert Day1.string_to_number("nine") == 9
    assert Day1.string_to_number("zero") == 0
  end

end
