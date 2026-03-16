using CymbalCore.CymbalLogging;
using Godot;

public class CymbalLogger : ILogger
{
    public LogLevel LogLevel { get; set; }

    public void Print(string s)
    {
        GD.Print(s);
    }

    public void PrintErr(string s)
    {
        GD.PrintErr(s);
    }
}