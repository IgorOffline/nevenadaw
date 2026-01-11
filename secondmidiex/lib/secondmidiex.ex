defmodule SecondMidiex do
  use Application

  def start(_type, _args) do
    IO.puts("Starting SecondMidiex application...")
    Task.start(fn -> play_midi() end)
    {:ok, self()}
  end

  def play_midi do
    conn = Midiex.create_virtual_output("ElixirOut")
    
    IO.puts("Created virtual port: #{conn.name}")
    
    Process.sleep(200)
    auto_connect()

    forever_loop(conn)
  end

  defp auto_connect do
    case System.cmd("aconnect", ["ElixirOut", "VirMIDI 2-0"]) do
      {_, 0} -> IO.puts("ALSA Patch Successful: ElixirOut -> VirMIDI 2-0")
      {_, _} -> IO.puts("Patch failed. Try: aconnect ElixirOut 'VirMIDI 2-0'")
    end
  end

  defp forever_loop(conn) do
    IO.puts("Sending C4 to Vital...")
    Midiex.send_msg(conn, <<0x90, 60, 100>>)
    Process.sleep(500)
    
    Midiex.send_msg(conn, <<0x80, 60, 0>>)
    Process.sleep(1500)
    
    forever_loop(conn)
  end
end
