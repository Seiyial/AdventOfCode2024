defmodule AOC.Day1 do
  def solve_1(input: String) do
    input
    |> String.split()
    |> Enum.chunk_every(2)
    |> Enum.zip()
    |> Enum.map(Enum.sort())
  end
end
