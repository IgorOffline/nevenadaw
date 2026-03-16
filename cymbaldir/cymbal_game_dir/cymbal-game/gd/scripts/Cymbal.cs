using CymbalCore.Cymbal;
using CymbalCore.CymbalWeb;
using Godot;

public partial class Cymbal : Node2D
{
	private CymbalLogger? _logger;

	public override void _Ready()
	{
		_logger = new CymbalLogger();
		var regina = new CymbalWeb().Get();
		_logger.Print($"[Cymbal 0.1.0] Regina Success: {regina.CymbalConfig?.Success?.ToString() ?? "(-)"}");

		if (regina.CymbalConfig != null)
		{
			if (regina.ImageBlue != null) CreateImageNode(regina.ImageBlue, new Vector2(200, 600), "Blue");
			if (regina.ImageGreen != null) CreateImageNode(regina.ImageGreen, new Vector2(400, 600), "Green");
			if (regina.ImageRed != null) CreateImageNode(regina.ImageRed, new Vector2(600, 600), "Red");
		}

		var icon = GetNodeOrNull<Sprite2D>("Icon");
		if (icon != null)
			_logger.Print($"Icon found at: {icon.Position}");
		else
			_logger.PrintErr("Icon node not found!");
	}

	private void CreateImageNode(CymbalImageConfig config, Vector2 position, string name)
	{
		if (string.IsNullOrEmpty(config.ImageUrl))
		{
			_logger!.PrintErr($"ImageUrl for {name} is null or empty!");
			return;
		}

		var web = new CymbalWeb();
		var (bytes, sha384) = web.GetImageBytes(config.ImageUrl);

		if (bytes == null || bytes.Length == 0)
		{
			_logger!.PrintErr($"Failed to fetch image {name} from {config.ImageUrl}");
			return;
		}

		if (!string.IsNullOrEmpty(config.ImageSha384) && !string.IsNullOrEmpty(sha384))
		{
			if (config.ImageSha384 != sha384)
			{
				_logger!.PrintErr($"SHA384 mismatch for {name}: {config.ImageSha384} != {sha384}");
				return;
			}

			_logger!.Print($"SHA384 verified for {name}: {sha384}");
		}

		var image = new Image();
		var error = image.LoadPngFromBuffer(bytes);
		if (error != Error.Ok)
		{
			_logger!.PrintErr($"Failed to load PNG for {name} from buffer: {error}");
			return;
		}

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
