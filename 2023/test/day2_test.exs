defmodule Day2Test do
  use ExUnit.Case

  test "part 1 color count valid" do
    constraint = [{:red, 12}, {:green, 13}, {:blue, 14}]

    color_count = {:blue, 3}
    assert Day2.Part1.color_count_valid(color_count, constraint) === true

    color_count = {:blue, 15}
    assert Day2.Part1.color_count_valid(color_count, constraint) === false
  end

  test "part 1 hand valid" do
    constraint = [{:red, 12}, {:green, 13}, {:blue, 14}]

    hand = [{:blue, 3}, {:red, 11}]
    assert Day2.Part1.hand_valid(hand, constraint) === true

    hand = [{:blue, 3}, {:red, 15}]
    assert Day2.Part1.hand_valid(hand, constraint) === false
  end

  test "part 1 game valid" do
    constraint = [{:red, 12}, {:green, 13}, {:blue, 14} ]

    game1 = {1, [[{:blue, 3}, {:red, 4}], [{:red, 1}, {:green, 2}, {:blue, 6}], [{:green, 2}]]}
    assert Day2.Part1.game_possible(game1, constraint) === true

    game2 = {2, [[{:blue, 1}, {:green, 2}], [{:green, 3}, {:blue, 4}, {:red, 1}], [{:green, 1}, {:blue, 1}]]}
    assert Day2.Part1.game_possible(game2, constraint) === true

    game3 = {3, [[{:green, 8}, {:blue, 6}, {:red, 20}], [{:blue, 5}, {:red, 4}, {:green, 13}], [{:green, 5}, {:red, 1}]]}
    assert Day2.Part1.game_possible(game3, constraint) === false

    game4 = {4, [[{:green, 1}, {:red, 3}, {:blue, 6}], [{:green, 3}, {:red, 6}], [{:green, 3}, {:blue, 15}, {:red, 14}]]}
    assert Day2.Part1.game_possible(game4, constraint) === false

    game5 = {5, [[{:red, 6}, {:blue, 1}, {:green, 3}], [{:blue, 2}, {:red, 1}, {:green, 2}]]}
    assert Day2.Part1.game_possible(game5, constraint) === true
  end

  test "part 1 test" do
    constraint = [{:red, 12}, {:green, 13}, {:blue, 14}]
    assert 8 ==
      AdventOfCode.get_puzzle_test(2, 1)
      |> Day2.Part1.solve(constraint)
  end

  test "part 1" do
    constraint = [{:red, 12}, {:green, 13}, {:blue, 14}]

    assert 1931 == AdventOfCode.get_puzzle(2)
      |> Day2.Part1.solve(constraint)
  end

  test "part 2 test" do
    assert 2286 == AdventOfCode.get_puzzle_test(2, 1)
    |> Day2.Part2.solve
  end

  test "part 2" do
    assert 83105 == AdventOfCode.get_puzzle(2)
    |> Day2.Part2.solve
  end
end
