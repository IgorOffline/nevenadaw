defmodule Thirdmidiex do
  use Application

  def start(_type, _args) do
    IO.puts("Thirdmidiex pt. 3")

    children = [
      {Task, fn -> run() end}
    ]

    opts = [strategy: :one_for_one, name: Thirdmidiex.Supervisor]
    Supervisor.start_link(children, opts)
  end

  def run do
    run(0)
  end

  defp run(i) do
    IO.puts(i)
    :timer.sleep(1000)
    run(i + 1)
  end
end