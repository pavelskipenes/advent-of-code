defmodule Day1Test do
  use ExUnit.Case

  test "to_integer" do
    assert 1 == Day1.to_integer([1])
    assert 12 == Day1.to_integer([1, 2])
    assert 123 == Day1.to_integer([1, 2, 3])
  end

  test "part 1 test" do
    assert 142 == AdventOfCode.get_puzzle_test(1,1)
    |> Day1.Part1.solve
  end

  test "part 1 prod" do
    assert 55208 ==
      AdventOfCode.get_puzzle(1)
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
      AdventOfCode.get_puzzle_test(1,2)
      |> Day1.Part2.solve()
  end

  test "part 2 prod" do
   value =
      AdventOfCode.get_puzzle(1)
      |> Day1.Part2.solve()

    assert value > 54558 # characters cannot be shared
    assert value == 54578 # characters can be shared
  end
end
