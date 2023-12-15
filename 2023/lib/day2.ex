defmodule Day2 do
  defmodule Part1 do
    def solve(input, constraint) when is_binary(input) when is_list(constraint) do
      input
      |> String.split("\n", trim: true)
      |> Enum.map(&game_parser/1)
      |> Enum.filter(fn game -> game_possible(game, constraint) end)
      |> Enum.map(fn game -> game[:id] end)
      |> Enum.sum
    end

    def game_possible(game, constraint) when is_map(game) and is_list(constraint) do
      invalid_samples = game[:samples]
        |> Enum.filter(fn sample -> not sample_valid(sample, constraint) end)
        |> Enum.count

      invalid_samples === 0
    end

    def sample_valid(sample, constraint) when is_list(sample) and is_list(constraint) do
      cond do
        Keyword.has_key?(sample, :red) and Keyword.get(sample, :red) > Keyword.get(constraint, :red) -> false
        Keyword.has_key?(sample, :green) and Keyword.get(sample, :green) > Keyword.get(constraint,:green) -> false
        Keyword.has_key?(sample, :blue) and Keyword.get(sample, :blue) >  Keyword.get(constraint,:blue) -> false
        true -> true
      end
    end

    def game_parser(game) when is_binary(game) do
      samples = game
        |> String.split(":")
        |> Enum.reverse()
        |> List.first()
        |> extract_game_samples

      id = game
        |> String.split(":")
        |> List.first()
        |> extract_id

      %{:id => id, :samples => samples}
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
      |> Enum.map(&count_cubes/1)
      |> Enum.map(fn element ->
        {color, {amount, _}} = element
        {color, amount}
      end)
    end

    def count_cubes(color_and_count) when is_binary(color_and_count) do
      count =
        color_and_count
        |> String.split(" ")
        |> List.first()

      color = String.slice(color_and_count, (String.length(count) + 1)..-1)

      case color do
        "red" -> {:red, Integer.parse(count)}
        "green" -> {:green, Integer.parse(count)}
        "blue" -> {:blue, Integer.parse(count)}
      end
    end

    def extract_id(game_part) when is_binary(game_part) do
      case game_part do
        "Game " <> id -> id
      end
      |> Integer.parse
      |> Kernel.then(fn result -> {val, _} = result; val end)
    end
  end

  defmodule Part2 do
    def solve(input, constraint) when is_binary(input) and is_list(constraint) do
    end
  end
end
