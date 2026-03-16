using CymbalCore.Cymbal;
using CymbalCore.CymbalWeb;
using Godot;

public partial class Cymbal : Node2D
{
	private readonly CymbalConfigLoader _loader = new();
	private CymbalLogger? _logger;
	private CymbalWeb? _web;

	public override void _Ready()
	{
		_logger = new CymbalLogger();
		_web = new CymbalWeb(_loader);
		var regina = _web.Get();
		_logger.Print($"[Cymbal 0.1.0] Regina Success: {regina.CymbalConfig!.Success}");

		if (regina.ImageBlue != null) CreateImageNode(regina.ImageBlue, new Vector2(200, 600), "Blue");
		if (regina.ImageGreen != null) CreateImageNode(regina.ImageGreen, new Vector2(400, 600), "Green");
		if (regina.ImageRed != null) CreateImageNode(regina.ImageRed, new Vector2(600, 600), "Red");

		var icon = GetNodeOrNull<Sprite2D>("Icon");
		if (icon != null)
			_logger.Print($"Icon found at: {icon.Position}");
		else
			_logger.PrintErr("Icon node not found!");
	}

	private void CreateImageNode(CymbalImageConfig config, Vector2 position, string name)
	{
		var (bytes, sha384) = _web!.GetImageBytes(config.ImageUrl);

		if (!string.IsNullOrEmpty(config.ImageSha384) && !string.IsNullOrEmpty(sha384))
		{
			if (config.ImageSha384 != sha384)
				throw new CymbalException($"SHA384 mismatch for {name}: {config.ImageSha384} != {sha384}");

			_logger!.Print($"SHA384 verified for {name}: {sha384}");
		}

		var image = new Image();
		var error = image.LoadPngFromBuffer(bytes);
		if (error != Error.Ok) throw new CymbalException($"Failed to load PNG for {name} from buffer: {error}");

		if (int.TryParse(config.ImageSampleWidth, out var sw) && int.TryParse(config.ImageSampleHeight, out var sh))
		{
			image.Resize(sw, sh);
			_logger!.Print($"Resized image {name} to sample size {sw}x{sh}");
		}
		else if (int.TryParse(config.ImageWidth, out var w) && int.TryParse(config.ImageHeight, out var h))
		{
			image.Resize(w, h);
			_logger!.Print($"Resized image {name} to {w}x{h}");
		}

		var texture = ImageTexture.CreateFromImage(image);
		var sprite = new Sprite2D();
		sprite.Texture = texture;
		sprite.Position = position;
		sprite.Name = $"DynamicImage_{name}";
		AddChild(sprite);

		_logger!.Print($"Created PNG Image Node '{name}' at {sprite.Position}");
	}

	public override void _Process(double delta)
	{
	}
}
