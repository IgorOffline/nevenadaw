using Godot;
using RustyMarchCore;

public partial class RustyMarchMain : Node2D
{
    public override void _Ready()
    {
        GD.Print("RustyMarchMain 0.1.0");
        var config = new RustyWeb().Get();
        var image = new RustyImage("+RustyImage", config.ImageUrl, config.ImageWidth, config.ImageHeight);
        GD.Print($"{image} {config.Success?.ToString() ?? "(-)"}");

        var icon = GetNodeOrNull<Sprite2D>("Icon");
        if (icon != null)
            GD.Print($"Icon found at: {icon.Position}");
        else
            GD.PrintErr("Icon node not found!");

        CreateImageNode(image);
    }

    private void CreateImageNode(RustyImage rustyImage)
    {
        if (string.IsNullOrEmpty(rustyImage.ImageUrl))
        {
            GD.PrintErr("ImageUrl is null or empty!");
            return;
        }

        var rustyWeb = new RustyWeb();
        var bytes = rustyWeb.GetImageBytes(rustyImage.ImageUrl);

        if (bytes == null || bytes.Length == 0)
        {
            GD.PrintErr($"Failed to fetch image from {rustyImage.ImageUrl}");
            return;
        }

        var image = new Image();
        var error = image.LoadPngFromBuffer(bytes);
        if (error != Error.Ok)
        {
            GD.PrintErr($"Failed to load PNG from buffer: {error}");
            return;
        }

        if (int.TryParse(rustyImage.ImageWidth, out var w) && int.TryParse(rustyImage.ImageHeight, out var h))
        {
            image.Resize(w, h);
            GD.Print($"Resized image to {w}x{h}");
        }

        var texture = ImageTexture.CreateFromImage(image);
        var sprite = new Sprite2D();
        sprite.Texture = texture;
        sprite.Position = new Vector2(750, 750);
        sprite.Name = "DynamicPngImage";
        AddChild(sprite);

        GD.Print($"Created PNG Image Node '{rustyImage.Name}' at {sprite.Position}");
    }

    public override void _Process(double delta)
    {
    }
}