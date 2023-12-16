defmodule Day3 do
  defmodule Part1 do
    def solve(input) when is_binary(input) do
    end

    @symbols ["*", "$", "#", "+"]
    @numbers ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
    def parse_shit(input) when is_binary(input) do
      {symbols, numbers, _} = input
        |> String.graphemes()
        |> Enum.reduce({[], [], {[], 0, 0}}, fn char, acumulator ->
          {symbols, numbers, {number_builder, line, i}} = acumulator

          case char do
            "." -> {symbols, gen_number(numbers, number_builder, line, i), {[], line, i + 1 }}
            symbol when symbol in @symbols -> {[{line, i} | symbols], gen_number(numbers, number_builder, line, i), {[], line, i + 1 }}
            "\n" -> {symbols, gen_number(numbers, number_builder, line, i), {[], line + 1, 0 }}
            num when num in @numbers -> {symbols, numbers, {[num | number_builder], line, i + 1}}
            character -> raise "unexpected character: \"#{character}\" detected at #{line}:#{i}"
          end
        end)

      {symbols, numbers}
    end

    @doc """
      Convert a list of numerical characters into a number and prepend it to `numbers`

      `numbers` is a list of accumulated numbers

      `number_builder` is a t:list of numbers as characters. This list is expected to be in reverse order. If `number_builder` is `nil` then `numbers` is returned unchanged

      `line` and `i` contains the indexies of the last character.
      """
    def gen_number(numbers, number_builder, line, i) when is_list(numbers) and (is_list(number_builder) or number_builder == nil) and is_integer(line) and is_integer(i) do
      case number_builder do
        [] -> numbers
        _ -> [{{line, i - Enum.count(number_builder)}, number_builder |> Enum.reverse |> Day1.to_integer} | numbers]
      end
    end
  end

  defmodule Part2 do
    def solve(input) when is_binary(input) do
    end
  end
end
