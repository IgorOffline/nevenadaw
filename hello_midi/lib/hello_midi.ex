defmodule HelloMidi do
  use Application

  def start(_type, _args) do
    IO.puts "Hello, Midi!"
    Supervisor.start_link([], strategy: :one_for_one)
  end
end
