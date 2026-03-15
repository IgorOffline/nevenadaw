using RustyMarchFrontendCore;

namespace RustyMarchFrontend;

internal class Program
{
    private static void Main(string[] args)
    {
        var rustyPoint = new RustyPoint(10, 20);
        Console.WriteLine("Hello {0} World!", rustyPoint);
    }
}