defmodule SecondMidiex do
  use Application

  def start(_type, _args) do
    IO.puts("SecondMidiex")
    Task.start(fn -> play_midi() end)
    {:ok, self()}
  end

  def play_midi do
    conn = Midiex.create_virtual_output("ElixirOut")

    Process.sleep(500)

    ["24:0", "14:0"] |> Enum.each(fn port ->
      System.cmd("aconnect", ["MIDIex:0", port])
    end)

    sequence_loop(conn)
  end

  defp sequence_loop(conn) do
    [60]
    |> Enum.each(fn note ->
      play_note(conn, note, 40, 200)
    end)

    Process.sleep(400)
    sequence_loop(conn)
  end

  defp play_note(conn, note, vel, duration) do
    Midiex.send_msg(conn, <<0x90, note, vel>>)
    Process.sleep(duration)
    Midiex.send_msg(conn, <<0x80, note, 0>>)
  end
end
