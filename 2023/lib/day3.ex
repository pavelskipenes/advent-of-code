defmodule Day3 do
  defmodule Part1 do
    def solve(input) when is_binary(input) do
    end

    def extract_symbols(line) when is_binary(line) do
      {indexies, _} =
        line
        |> String.graphemes()
        |> Enum.reduce({[], 0}, fn character, acumulator ->
          {indexies, iteration} = acumulator

          indexies =
            case character do
              "*" -> [iteration | indexies]
              "$" -> [iteration | indexies]
              "#" -> [iteration | indexies]
              "+" -> [iteration | indexies]
              _ -> indexies
            end

          {indexies, iteration + 1}
        end)

      indexies
    end
  end

  defmodule Part2 do
    def solve(input) when is_binary(input) do
    end
  end
end
