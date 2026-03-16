namespace CymbalCore.CymbalLogging;

public interface ILogger
{
    LogLevel LogLevel { get; set; }

    void Trace(string s)
    {
        if ((ushort)LogLevel.Trace >= (ushort)LogLevel) Print(s);
    }

    void Info(string s)
    {
        if ((ushort)LogLevel.Info >= (ushort)LogLevel) Print(s);
    }

    void Print(string s);
}