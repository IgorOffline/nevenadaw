using Godot;

public partial class Cymbal : Node2D
{
	public override void _Ready()
	{
		var logger = new CymbalLogger();
		logger.Print("Cymbal 0.1.0");
	}

	public override void _Process(double delta)
	{
	}
}
