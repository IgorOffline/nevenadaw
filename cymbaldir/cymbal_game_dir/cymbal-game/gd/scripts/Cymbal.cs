using CymbalCore.Cymbal;
using CymbalCore.CymbalWeb;
using Godot;

public partial class Cymbal : Node2D
{
	private CymbalLogger? _logger;
	private CymbalRegina? _regina;
	private CymbalWeb? _web;

	public override void _Ready()
	{
		_logger = new CymbalLogger();
		_regina = new CymbalConfigLoader().Get();
		_web = new CymbalWeb(_regina);
		var regina = _web.Get();
		_logger.Print($"[Cymbal 0.1.0] Regina Success: {regina.CymbalConfig!.Success}");

		if (regina.ImageBlue != null) CreateImageNode(regina.ImageBlue, new Vector2(80, 60), "Blue");
		if (regina.ImageGreen != null) CreateImageNode(regina.ImageGreen, new Vector2(150, 60), "Green");
		if (regina.ImageRed != null) CreateImageNode(regina.ImageRed, new Vector2(220, 60), "Red");
		CreateButtonNode("Lorem", "Ipsum", new Vector2(50, 150), new Vector2(120, 60));

		var icon = GetNodeOrNull<Sprite2D>("Icon");
		if (icon != null)
			_logger.Print($"Icon found at: {icon.Position}");
		else
			_logger.PrintErr("Icon node not found!");
	}

	private void CreateImageNode(CymbalImageConfig config, Vector2 position, string name)
	{
		var (bytes, sha384) = _web!.GetImageBytes($"{_regina!.CymbalConfig!.Url!}{config.ImageUrl}");

		if (!string.IsNullOrEmpty(config.ImageSha384) && !string.IsNullOrEmpty(sha384))
		{
			if (config.ImageSha384 != sha384)
				throw new CymbalException($"SHA384 mismatch for {name}: {config.ImageSha384} != {sha384}");

			_logger!.Print($"SHA384 verified for {name}: {sha384}");
		}

		var image = new Image();
		var error = image.LoadPngFromBuffer(bytes);
		if (error != Error.Ok) throw new CymbalException($"Failed to load PNG for {name} from buffer: {error}");
		image.Resize(60, 60);
		var texture = ImageTexture.CreateFromImage(image);
		var sprite = new Sprite2D();
		sprite.Texture = texture;
		sprite.Position = position;
		sprite.Name = $"DynamicImage_{name}";
		AddChild(sprite);

		_logger!.Print($"Created PNG Image Node '{name}' at {sprite.Position}");
	}

	private void CreateButtonNode(string text, string logMessage, Vector2 position, Vector2 size)
	{
		var button = new Button();
		button.Text = text;
		button.Position = position;
		button.Size = size;
		button.Pressed += () => _logger!.Print(logMessage);
		button.Name = $"DynamicButton_{text}";
		AddChild(button);

		_logger!.Print($"Created Button Node '{text}' at {button.Position} with size {button.Size}");
	}

	public override void _Process(double delta)
	{
	}
}
