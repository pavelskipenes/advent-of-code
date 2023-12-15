defmodule Day2Test do
  use ExUnit.Case

  test "part 1 sample invalid" do
    constraint = [{:red, 12}, {:green, 13}, {:blue, 14}]

    sample = [{:blue, 3}, {:red, 4}]
    assert Day2.Part1.sample_valid(sample, constraint) === true

    sample = [{:red, 1}, {:green, 2}, {:blue, 16}]
    assert Day2.Part1.sample_valid(sample, constraint) === false

    sample = [{:green, 2}]
    assert Day2.Part1.sample_valid(sample, constraint) === true
  end

  test "part 1 game possible" do
    constraint = [red: 12, green: 13, blue: 14 ]

    game1 = %{id: 1, samples: [[blue: 3, red: 4], [red: 1, green: 2, blue: 6], [green: 2]]}
    assert Day2.Part1.game_possible(game1, constraint) === true

    game2 = %{id: 2, samples: [[blue: 1, green: 2], [green: 3, blue: 4, red: 1], [green: 1, blue: 1]]}
    assert Day2.Part1.game_possible(game2, constraint) === true

    game3 = %{id: 3, samples: [[green: 8, blue: 6, red: 20], [blue: 5, red: 4, green: 13], [green: 5, red: 1]]}
    assert Day2.Part1.game_possible(game3, constraint) === false

    game4 = %{id: 4, samples: [[green: 1, red: 3, blue: 6], [green: 3, red: 6], [green: 3, blue: 15, red: 14]]}
    assert Day2.Part1.game_possible(game4, constraint) === false

    game5 = %{id: 5, samples: [[red: 6, blue: 1, green: 3], [blue: 2, red: 1, green: 2]]}
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
