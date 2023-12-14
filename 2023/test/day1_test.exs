defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "test puzzle one" do
    {result, file_content} = File.read("puzzle_input/day_1/part1_test.txt")
    assert result == :ok
    assert 142 == Day1.Part1.solve(file_content)
  end

  test "problem 1" do
    assert 55208 ==
      File.read!("puzzle_input/day_1/input.txt")
      |> Day1.Part1.solve()
  end

  test "number matching" do
    assert Day1.Part2.string_to_number("one") == {:ok, 1}
    assert Day1.Part2.string_to_number("two") == {:ok, 2}
    assert Day1.Part2.string_to_number("three") == {:ok, 3}
    assert Day1.Part2.string_to_number("four") == {:ok, 4}
    assert Day1.Part2.string_to_number("five") == {:ok, 5}
    assert Day1.Part2.string_to_number("six") == {:ok, 6}
    assert Day1.Part2.string_to_number("seven") == {:ok, 7}
    assert Day1.Part2.string_to_number("eight") == {:ok, 8}
    assert Day1.Part2.string_to_number("nine") == {:ok, 9}
  end

  test "windows size" do
    assert 0..1 == Day1.Part2.windows_slice_indexies([1, 2], 1)
    assert 0..0 == Day1.Part2.windows_slice_indexies([1, 2], 2)
    assert 0..4 == Day1.Part2.windows_slice_indexies([1, 2, 3, 4, 5, 6], 2)
  end

  test "windows" do
    [[1, 2], [2, 3], [3, 4], [4, 5], [5, 6]] = Day1.Part2.windows([1, 2, 3, 4, 5, 6], 2)
  end

  test "to_integer" do
    assert 1 == Day1.to_integer([1])
    assert 12 == Day1.to_integer([1, 2])
    assert 123 == Day1.to_integer([1, 2, 3])
  end

  test "extract numbers" do
    value = "123"
      |> String.graphemes
      |> Day1.Part2.extract_numbers
      |> Enum.map(fn number -> Integer.parse(number) end)
      |> Enum.map(fn {val, ""} -> val end)
      |> Day1.to_integer
    assert value == 123
    value = "one"
      |> String.graphemes
      |> Day1.Part2.extract_numbers
      |> IO.inspect
      |> Enum.map(fn number -> Integer.parse(number) end)
      |> Enum.map(fn {val, ""} -> val end)
      |> Day1.to_integer
    assert value == 1

    value = "2oneightg"
      |> String.graphemes()
      |> Day1.Part2.extract_numbers
      |> IO.inspect
      |> Enum.map(fn number -> Integer.parse(number) end)
      |> Enum.map(fn {val, ""} -> val end)
      |> Day1.to_integer
    assert value == ["2", "1", "8"]
  end

  # test "part 2 test input" do
  #   assert 281 ==
  #            File.read!("puzzle_input/day_1/part2_test.txt")
  #            |> Day1.Part2.solve()
  # end
  #
  # test "part 2 prod input" do
  #   assert 54558 <
  #            File.read!("puzzle_input/day_1/input.txt")
  #            |> Day1.Part2.solve()
  # end
end
