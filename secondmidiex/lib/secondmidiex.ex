defmodule SecondMidiex do
  def main(_args \\ []) do
    case run() do
      :ok -> :ok
      {:error, reason} -> IO.puts("Error: #{reason}")
    end
  end

  def run do
    IO.puts("SecondMidiex")
    ports = Midiex.ports(:output)

    case length(ports) do
      0 ->
        {:error, "no output port found"}

      1 ->
        IO.puts("Choosing the only available output port: #{hd(ports).name}")
        connect_and_play(ports)

      _ ->
        IO.puts("\nAvailable output ports:")
        ports |> Enum.with_index() |> Enum.each(fn {p, i} -> IO.puts("#{i}: #{p.name}") end)
        connect_and_play(ports)
    end
  end

  defp connect_and_play(ports) do
    port = Enum.find(ports, fn p -> String.contains?(p.name, "FLUID Synth") end) || hd(ports)
    conn = Midiex.open(port)

    IO.puts("Connected to: #{port.name}")

    notes = [60, 62, 64, 65, 67, 69, 71, 72]

    Enum.each(notes, fn note ->
      Midiex.send_msg(conn, <<0x90, note, 127>>)
      IO.puts("Playing note: #{note}")
      :timer.sleep(500)
      Midiex.send_msg(conn, <<0x80, note, 127>>)
    end)

    Midiex.close(conn)
    IO.puts("Connection closed")

    :ok
  end
end
