defmodule Secondmidiex.MixProject do
  use Mix.Project

  def project do
    [
      app: :secondmidiex,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      escript: [main_module: SecondMidiex],
      default_task: "run"
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {SecondMidiex, []}
    ]
  end

  defp deps do
    [
      {:midiex, "~> 0.6.3"}
    ]
  end
end
