defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "to_integer" do
    assert 1 == Day1.to_integer([1])
    assert 12 == Day1.to_integer([1, 2])
    assert 123 == Day1.to_integer([1, 2, 3])
  end

  test "part 1 test" do
    {result, file_content} = File.read("puzzle_input/day_1/part1_test.txt")
    assert result == :ok
    assert 142 == Day1.Part1.solve(file_content)
  end

  test "part 1 prod" do
    assert 55208 ==
      File.read!("puzzle_input/day_1/input.txt")
      |> Day1.Part1.solve()
  end

  test "extract numbers 1" do
    value = ~c"123_456_789"
      |> Day1.Part2.extract_numbers
      |> Day1.to_integer
    assert value == 123_456_789

    value = ~c"one_two_three_four_five_six_seven_eight_nine"
      |> Day1.Part2.extract_numbers
      |> Day1.to_integer
    assert value == 123_456_789

    value = ~c"oneight"
      |> Day1.Part2.extract_numbers
      |> Day1.to_integer
    assert value == 1
  end

  test "extract numbers 2" do
    value = ~c"123456789"
      |> Day1.Part2.extract_numbers_shared
      |> Day1.to_integer
    assert value == 123_456_789

    value = ~c"one_two_three_four_five_six_seven_eight_nine"
      |> Day1.Part2.extract_numbers_shared
      |> Day1.to_integer
    assert value == 123_456_789

    value = ~c"oneight"
      |> Day1.Part2.extract_numbers_shared
      |> Day1.to_integer
    assert value == 18
  end

  test "part 2 test" do
    assert 281 ==
      File.read!("puzzle_input/day_1/part2_test.txt")
      |> Day1.Part2.solve()
  end

  test "part 2 prod" do
   value =
      File.read!("puzzle_input/day_1/input.txt")
      |> Day1.Part2.solve()

    assert value > 54558 # characters cannot be shared
    assert value == 54578 # characters can be shared
  end
end
