Mix.install([
  {:bandit, "~> 1.5"},
  {:jason, "~> 1.2"},
  {:phoenix, "~> 1.8.0"},
  {:phoenix_html, "~> 4.1"},
  {:phoenix_live_view, "~> 1.1.0"},
  {:midiex, "~> 0.6.3"}, # ?
])

Application.put_env(:winter_elixir, WinterElixir.Endpoint,
  http: [port: 4000],
  server: true,
  adapter: Bandit.PhoenixAdapter,
  render_errors: [formats: [html: WinterElixir.ErrorHTML], layout: false],
  pubsub_server: WinterElixir.PubSub,
  secret_key_base: String.duplicate("a", 64)
)

defmodule WinterElixir.ErrorHTML do
  def render(template, _assigns) do
    Phoenix.Controller.status_message_from_template(template)
  end
end

defmodule WinterElixir.Router do
  use Phoenix.Router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
  end

  scope "/" do
    pipe_through :browser
    get "/", WinterElixir.TwentyController, :index
    get "/roll", WinterElixir.TwentyController, :roll
    get "/index.css", WinterElixir.TwentyController, :style
  end
end

defmodule WinterElixir.Endpoint do
  use Phoenix.Endpoint, otp_app: :winter_elixir

  plug Plug.Session,
       store: :cookie,
       key: "_winter_elixir_key",
       signing_salt: "nZ6Lz6M/"

  plug WinterElixir.Router
end

defmodule WinterElixir.Dice do
  def roll do
    Enum.random(1..6)
  end
end

defmodule WinterElixir.TwentyController do
  use Phoenix.Controller, formats: [:html, :json]

  def index(conn, _params) do
    roll = WinterElixir.Dice.roll()
    content = EEx.eval_file("index.html", [assigns: %{roll: roll}])
    html(conn, content)
  end

  def roll(conn, _params) do
    json(conn, %{roll: WinterElixir.Dice.roll()})
  end

  def style(conn, _params) do
    conn
    |> put_resp_content_type("text/css")
    |> text(File.read!("index.css"))
  end
end

if System.get_env("WINTER_ELIXIR_START_SERVER") == "true" do
  {:ok, _} = Application.ensure_all_started(:phoenix)
  _ = Phoenix.PubSub.Supervisor.start_link(name: WinterElixir.PubSub, adapter: Phoenix.PubSub.PG2)
  WinterElixir.Endpoint.start_link()
  Process.sleep(:infinity)
end
