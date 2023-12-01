defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "greets the world" do
    assert Day1.hello() == :world
  end

  test "read puzzle input" do
    assert {:ok, _} = Day1.read_file()
  end
end
