Code.require_file("app.exs")

ExUnit.start()

defmodule WinterElixir.DiceTest do
  use ExUnit.Case, async: true

  alias WinterElixir.Dice

  @tag :unit
  test "Dice.roll/0 returns a value between 1 and 6" do
    rolls = Enum.map(1..5000, fn _ -> Dice.roll() end)

    min = Enum.min(rolls)
    max = Enum.max(rolls)

    assert min == 1, "Expected minimum value to be 1, got #{min}"
    assert max == 6, "Expected maximum value to be 6, got #{max}"
  end
end
