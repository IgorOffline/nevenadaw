defmodule Mix.Tasks.Secondmidiex do
  use Mix.Task

  @shortdoc "Runs the SecondMidiex application"

  def run(_args) do
    SecondMidiex.main([])
  end
end
