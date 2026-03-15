using Godot;
using RustyMarchCore;

public partial class RustyMarchMain : Node2D
{
    public override void _Ready()
    {
        GD.Print("RustyMarchMain 0.1.0");
        GD.Print(new RustyImage("+RustyImage"));

        var icon = GetNodeOrNull<Sprite2D>("Icon");
        if (icon != null)
        {
            GD.Print($"Icon found at: {icon.Position}");
        }
        else
        {
            GD.PrintErr("Icon node not found!");
        }
    }

    public override void _Process(double delta)
    {
    }
}