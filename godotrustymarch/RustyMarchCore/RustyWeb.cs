using Tomlyn;
using Tomlyn.Model;

namespace RustyMarchCore;

public class RustyWeb
{
    private static readonly HttpClient Client = new();

    public RustyConfig Get()
    {
        string? url = null;
        string? imageUrl = null;
        string? imageWidth = null;
        string? imageHeight = null;
        bool? success = null;

        try
        {
            const string configPath = @"C:\Users\igor\dev\nevenadawdir\nevenadaw\godotrustymarch\rustymarch.toml";

            if (File.Exists(configPath))
            {
                var content = File.ReadAllText(configPath);
                var model = TomlSerializer.Deserialize<TomlTable>(content);
                if (model != null && model.TryGetValue("rustymarch", out var sectionObj) &&
                    sectionObj is TomlTable section)
                {
                    if (section.TryGetValue("url", out var urlObj) && urlObj is string urlValue)
                        url = urlValue;
                    if (section.TryGetValue("image_url", out var imageUrlObj) && imageUrlObj is string imageUrlValue)
                        imageUrl = imageUrlValue;
                    if (section.TryGetValue("image_width", out var imageWidthObj) &&
                        imageWidthObj is string imageWidthValue)
                        imageWidth = imageWidthValue;
                    if (section.TryGetValue("image_height", out var imageHeightObj) &&
                        imageHeightObj is string imageHeightValue)
                        imageHeight = imageHeightValue;
                }
            }
            else
            {
                return new RustyConfig(null, null, null, null, null);
            }

            if (string.IsNullOrEmpty(url)) return new RustyConfig(url, imageUrl, imageWidth, imageHeight, null);

            using var request = new HttpRequestMessage(HttpMethod.Get, url);
            using var response = Client.Send(request);
            success = response.IsSuccessStatusCode;
        }
        catch
        {
            success = false;
        }

        return new RustyConfig(url, imageUrl, imageWidth, imageHeight, success);
    }
}