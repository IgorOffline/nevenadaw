defmodule SecondMidiex do
  use Application

  @impl true
  def start(_type, _args) do
    IO.puts("Starting SecondMidiex application...")
    Task.start(fn -> play_midi() end)
    {:ok, self()}
  end

  def play_midi do
    conn = Midiex.create_virtual_output("ElixirOut")
    IO.puts("Created virtual port: ElixirOut")

    Process.sleep(500)

    IO.puts("Manually bridging MIDIex to VirMIDI...")

    System.cmd("aconnect", ["MIDIex:0", "24:0"])

    System.cmd("aconnect", ["MIDIex:0", "14:0"])

    forever_loop(conn)
  end

  defp find_virmidi_address do
    Midiex.ports()
    |> Enum.find(fn p ->
      p.direction == :input && String.contains?(p.name, "2-0")
    end)
    |> case do
         nil -> nil
         port ->
           port.name |> String.split(" ") |> List.last()
       end
  end

  defp forever_loop(conn) do
    velocity = 35
    note = 60

    IO.puts("Sending quiet C4 (Vel: #{velocity}) to Vital...")

    Midiex.send_msg(conn, <<0x90, note, velocity>>)
    Process.sleep(500)

    Midiex.send_msg(conn, <<0x80, note, 0>>)
    Process.sleep(1000)

    forever_loop(conn)
  end
end