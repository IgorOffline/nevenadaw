defmodule SecondMidiex do
  use Application

  def start(_type, _args) do
    IO.puts("Starting SecondMidiex application")
    children = [
      {Task, fn -> play_midi() end}
    ]

    opts = [strategy: :one_for_one, name: SecondMidiex.Supervisor]
    Supervisor.start_link(children, opts)
  end

  def main(_args \\ []) do
    play_midi()
    :ok
  end

  def play_midi do
    virmidi_ports =
      Midiex.ports()
      |> Enum.filter(fn p ->
        p.direction == :output && String.contains?(p.name, "2-0")
      end)

    IO.puts("Found #{Enum.count(virmidi_ports)} sub-ports for Virtual Raw MIDI 2-0.")

    Enum.each(virmidi_ports, fn port ->
      IO.puts("Testing sub-port: #{port.name}...")

      conn = Midiex.open(port)

      if conn do
        Midiex.send_msg(conn, <<0x90, 60, 100>>)
        Process.sleep(500)

        Midiex.send_msg(conn, <<0x80, 60, 0>>)
        Process.sleep(200)
      else
        IO.puts("Failed to connect to #{port.name}")
      end
    end)

    IO.puts("Complete!")
  end
end