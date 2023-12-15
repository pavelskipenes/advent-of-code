defmodule Day2 do
  defmodule Part1 do
    def solve(input, constraint) when is_binary(input) when is_list(constraint) do
      input
      |> String.split("\n", trim: true)
      |> Enum.map(&game_parser/1)
      |> Enum.filter(fn game -> game_possible(game, constraint) end)
      |> Enum.map(fn game -> {id, _samples} = game; id end)
      |> Enum.sum
    end

    def game_parser(game) when is_binary(game) do
      game_samples = game |> String.split(":")

      id = game_samples
        |> List.first()
        |> extract_id

      samples = game_samples
        |> Enum.reverse()
        |> List.first()
        |> extract_game_samples

      {id, samples}
    end

    def extract_id(game_part) when is_binary(game_part) do
      case game_part do
        "Game " <> id -> id
      end
      |> Integer.parse
      |> Kernel.then(fn result -> {val, _} = result; val end)
    end

    def extract_game_samples(game_samples) when is_binary(game_samples) do
      game_samples
      |> String.trim()
      |> String.split(";")
      |> Enum.map(&extract_game_sample/1)
    end

    def extract_game_sample(input) when is_binary(input) do
      input
      |> String.split(",")
      |> Enum.map(&String.trim/1)
      |> Enum.map(&color_count/1)
    end

    def color_count(color_and_count) when is_binary(color_and_count) do
      [ count | color_text ] = color_and_count |> String.split(" ")
      {count, _} = Integer.parse(count)

      case List.first(color_text) do
        "red" -> {:red, count}
        "green" -> {:green, count}
        "blue" -> {:blue, count}
      end
    end

    def game_possible(game, constraint) when is_tuple(game) and is_list(constraint) and tuple_size(game) == 2 do
      {_id, samples} = game
      invalid_samples = samples
        |> Enum.filter(fn samples -> not hand_valid(samples, constraint) end)
        |> Enum.count

      invalid_samples === 0
    end

    def hand_valid(hand, constraints) when is_list(hand) and is_list(constraints) do
      0 === hand
        |> Enum.filter(fn color_amount -> not color_count_valid(color_amount, constraints) end)
        |> Enum.count
    end

    def color_count_valid(color_count, constraints) when is_tuple(color_count) and tuple_size(color_count) == 2 and is_list(constraints) do
      {color, count} = color_count
      cond do
        Keyword.has_key?(constraints, color) -> not (count > Keyword.get(constraints, color))
        true -> true
      end
    end
  end

  defmodule Part2 do
    def solve(input) when is_binary(input) do
      input
      |> String.split("\n", trim: true)
      |> Enum.map(&Part1.game_parser/1)
      |> Enum.map(&min_sample_set/1)
      |> Enum.map(&power/1)
      |> Enum.sum
    end

    def min_sample_set(game) when is_tuple(game) and tuple_size(game) == 2 do
      accumulator = [{:red, nil}, {:green, nil}, {:blue, nil}]

      {_id, samples} = game
      samples
      |> List.flatten
      |> Enum.reduce(accumulator,
        fn color_amount, accumulator ->
          accumulator |> Enum.map(fn constraint ->
            case {constraint, color_amount} do
              {{color, nil}, {color, amount}} when is_atom(color) and is_integer(amount) -> {color, amount}
              {{color, amount1}, {color, amount2}} when is_atom(color) and is_integer(amount1) and is_integer(amount2) -> {color, max(amount1, amount2)}
              _ -> constraint
            end
          end)
      end)
    end

    def power(max_values) when is_list(max_values) do
      max_values
      |> Enum.map(fn sample -> {_color, count} = sample; count end)
      |> Enum.reduce(1, fn el, acc -> acc*el end)
    end
  end
end
