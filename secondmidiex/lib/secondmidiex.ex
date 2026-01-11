defmodule SecondMidiex do
  def main(_args \\ []) do
    case run() do
      :ok -> :ok
      {:error, reason} -> IO.puts("Error: #{reason}")
    end
  end

  def run do
    IO.puts("Scanning for Virtual MIDI ports...")
    ports = Midiex.ports(:output)

    port = Enum.find(ports, fn p ->
      String.contains?(p.name, "VirMIDI") || String.contains?(p.name, "2-0")
    end)

    case port do
      nil ->
        IO.puts("Available ports were: #{inspect(Enum.map(ports, & &1.name))}")
        {:error, "Could not find VirMIDI port."}

      _ ->
        IO.puts("Connecting to: #{port.name}")
    end
  end
end
