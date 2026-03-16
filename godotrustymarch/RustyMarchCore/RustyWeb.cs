using Tomlyn;
using Tomlyn.Model;

namespace RustyMarchCore;

public class RustyWeb
{
    private static readonly HttpClient Client = new();

    public bool? Get()
    {
        try
        {
            const string configPath = @"C:\Users\igor\dev\nevenadawdir\nevenadaw\godotrustymarch\rustymarch.toml";
            string? url = null;

            if (File.Exists(configPath))
            {
                var content = File.ReadAllText(configPath);
                var model = TomlSerializer.Deserialize<TomlTable>(content);
                if (model != null && model.TryGetValue("rustymarch", out var sectionObj) &&
                    sectionObj is TomlTable section)
                    if (section.TryGetValue("url", out var urlObj) && urlObj is string urlValue)
                        url = urlValue;
            }
            else
            {
                return null;
            }

            if (string.IsNullOrEmpty(url)) return null;

            using var request = new HttpRequestMessage(HttpMethod.Get, url);
            using var response = Client.Send(request);
            return response.IsSuccessStatusCode;
        }
        catch
        {
            return false;
        }
    }
}