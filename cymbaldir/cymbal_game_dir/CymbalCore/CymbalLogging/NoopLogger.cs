namespace CymbalCore.CymbalLogging;

public class NoopLogger : ILogger
{
    public LogLevel LogLevel { get; set; }

    public void Print(string s)
    {
        // Noop
    }
}