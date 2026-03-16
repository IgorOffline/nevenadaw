using Godot;
using RustyMarchCore;

public partial class RustyMarchMain : Node2D
{
    public override void _Ready()
    {
        GD.Print("RustyMarchMain 0.1.0");
        var config = new RustyWeb().Get();
        GD.Print($"Config Success: {config.Success?.ToString() ?? "(-)"}");

        if (config.Regina != null)
        {
            if (config.Regina.ImageBlue != null)
                CreateImageNode(config.Regina.ImageBlue, new Vector2(600, 600), "Blue");
            if (config.Regina.ImageGreen != null)
                CreateImageNode(config.Regina.ImageGreen, new Vector2(700, 600), "Green");
            if (config.Regina.ImageRed != null) CreateImageNode(config.Regina.ImageRed, new Vector2(800, 600), "Red");
        }

        var icon = GetNodeOrNull<Sprite2D>("Icon");
        if (icon != null)
            GD.Print($"Icon found at: {icon.Position}");
        else
            GD.PrintErr("Icon node not found!");
    }

    private void CreateImageNode(RustyImageConfig config, Vector2 position, string name)
    {
        if (string.IsNullOrEmpty(config.ImageUrl))
        {
            GD.PrintErr($"ImageUrl for {name} is null or empty!");
            return;
        }

        var rustyWeb = new RustyWeb();
        var (bytes, sha384) = rustyWeb.GetImageBytes(config.ImageUrl);

        if (bytes == null || bytes.Length == 0)
        {
            GD.PrintErr($"Failed to fetch image {name} from {config.ImageUrl}");
            return;
        }

        if (!string.IsNullOrEmpty(config.ImageSha384) && !string.IsNullOrEmpty(sha384))
        {
            if (config.ImageSha384 != sha384)
            {
                GD.PrintErr($"SHA384 mismatch for {name}: {config.ImageSha384} != {sha384}");
                return;
            }

            GD.Print($"SHA384 verified for {name}: {sha384}");
        }

        var image = new Image();
        var error = image.LoadPngFromBuffer(bytes);
        if (error != Error.Ok)
        {
            GD.PrintErr($"Failed to load PNG for {name} from buffer: {error}");
            return;
        }

        if (int.TryParse(config.ImageWidth, out var w) && int.TryParse(config.ImageHeight, out var h))
        {
            image.Resize(w, h);
            GD.Print($"Resized image {name} to {w}x{h}");
        }

        var texture = ImageTexture.CreateFromImage(image);
        var sprite = new Sprite2D();
        sprite.Texture = texture;
        sprite.Position = position;
        sprite.Name = $"DynamicImage_{name}";
        AddChild(sprite);

        GD.Print($"Created PNG Image Node '{name}' at {sprite.Position}");
    }

    public override void _Process(double delta)
    {
    }
}