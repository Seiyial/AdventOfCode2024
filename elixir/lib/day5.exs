defmodule Ruleset do
  defstruct blacklist_lut: %{}, before_lut: %{}

  def add_rule(ruleset: Ruleset, left: Integer, right: Integer) do
    Map.update(ruleset.blacklist_lut, right, [left], fn rest -> [left | rest] end)
    Map.update(ruleset.before_lut, left, [right], fn rest -> [right | rest] end)
  end

  def from_rule_rows(rule_rows: List) do
    rule_rows
    |> Enum.map(&Ruleset.split_line_to_tuple/1)
    |> Enum.reduce(%Ruleset{}, fn {left, right}, acc ->
      Ruleset.add_rule(acc, left, right)
    end)
  end

  def split_line_to_tuple(line: String) do
    line |> String.split("|") |> Enum.map(&String.to_integer/1)
  end
end

defmodule Parser do
  @rule_regex ~r/\d+/

  defp parse_data_row(row: String) do
    row |> String.split(",") |> Enum.map(&String.to_integer/1)
  end

  defp get_rules_and_data_rows(input: String) do
    input |> String.split("\n\n") |> Enum.map(&String.split(&1, "\n", trim: true))
  end

  def parse_input(input: String) do
    input
    |> get_rules_and_data_rows()
    |> then(fn [rule_rows, data_rows] ->
      {
        Ruleset.from_rule_rows(rule_rows),
        Crawler.from_data_rows(data_rows)
      }
    end)
  end
end
