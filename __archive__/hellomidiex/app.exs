Mix.install([
  {:jason, "~> 1.4.4"},
  {:midiex, "~> 0.6.3"},
  {:rustler_precompiled, "~> 0.8.4"},
  {:castore, "~> 1.0.17"},
])

defmodule HelloMidi do
  def run do
    port_name = "ElixirBridge"

    port = Midiex.ports(:output)
           |> Enum.find(fn p -> String.contains?(p.name, port_name) end)

    case port do
      nil -> IO.puts("Could not find #{port_name}! Is loopMIDI running?")
      _ ->
        conn = Midiex.open(port)
        IO.puts("Connected to #{port.name} on Windows!")
        play_note(conn, 0, 49, 2000)
    end
  end
end

HelloMidi.run()