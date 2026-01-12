defmodule Thirdmidiex.MixProject do
  use Mix.Project

  def project do
    [
      app: :thirdmidiex,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {Thirdmidiex, []}
    ]
  end

  defp deps do
    [
      {:midiex, "~> 0.6.3"}
    ]
  end

  defp aliases do
    [
      run: "run --no-halt"
    ]
  end
end