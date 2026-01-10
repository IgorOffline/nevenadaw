defmodule HelloMidiTest do
  use ExUnit.Case
  doctest HelloMidi

  test "greets the world" do
    assert HelloMidi.hello() == :world
  end
end
