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

        string? sha384 = null;
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
                    if (section.TryGetValue("url", out var urlObj) && urlObj is string urlValue) url = urlValue;

                    var imageTables = section.Values.OfType<TomlTable>()
                        .Where(t => t.ContainsKey("image_url"))
                        .ToList();

                    if (imageTables.Count > 0)
                    {
                        var selectedImageTable = imageTables[Random.Shared.Next(imageTables.Count)];

                        if (selectedImageTable.TryGetValue("image_url", out var imageUrlObj) &&
                            imageUrlObj is string imageUrlValue)
                            imageUrl = imageUrlValue;
                        if (selectedImageTable.TryGetValue("image_width", out var imageWidthObj) &&
                            imageWidthObj is string imageWidthValue)
                            imageWidth = imageWidthValue;
                        if (selectedImageTable.TryGetValue("image_height", out var imageHeightObj) &&
                            imageHeightObj is string imageHeightValue)
                            imageHeight = imageHeightValue;
                        if (selectedImageTable.TryGetValue("image_sha384", out var sha384Obj) &&
                            sha384Obj is string sha384Value)
                            sha384 = sha384Value;
                    }
                    else
                    {
                        if (section.TryGetValue("image_url", out var imageUrlObj) &&
                            imageUrlObj is string imageUrlValue)
                            imageUrl = imageUrlValue;
                        if (section.TryGetValue("image_width", out var imageWidthObj) &&
                            imageWidthObj is string imageWidthValue)
                            imageWidth = imageWidthValue;
                        if (section.TryGetValue("image_height", out var imageHeightObj) &&
                            imageHeightObj is string imageHeightValue)
                            imageHeight = imageHeightValue;
                        if (section.TryGetValue("image_sha384", out var sha384Obj) && sha384Obj is string sha384Value)
                            sha384 = sha384Value;
                    }
                }
            }
            else
            {
                return new RustyConfig(null, null, null, null, null, null);
            }

            if (string.IsNullOrEmpty(url)) return new RustyConfig(url, imageUrl, imageWidth, imageHeight, sha384, null);

            using var request = new HttpRequestMessage(HttpMethod.Get, url);
            using var response = Client.Send(request);
            success = response.IsSuccessStatusCode;
            if (TryGetHeader(response, "x-sha384", out var value)) sha384 = value;
        }
        catch
        {
            success = false;
        }

        return new RustyConfig(url, imageUrl, imageWidth, imageHeight, sha384, success);
    }

    public (byte[]? Bytes, string? Sha384) GetImageBytes(string? url)
    {
        if (string.IsNullOrEmpty(url)) return (null, null);

        try
        {
            using var request = new HttpRequestMessage(HttpMethod.Get, url);
            using var response = Client.Send(request);
            if (response.IsSuccessStatusCode)
            {
                string? sha384 = null;
                if (TryGetHeader(response, "x-sha384", out var value)) sha384 = value;

                using var ms = new MemoryStream();
                response.Content.ReadAsStream().CopyTo(ms);
                return (ms.ToArray(), sha384);
            }
        }
        catch
        {
            throw new RustyException("Failed to fetch image from URL");
        }

        return (null, null);
    }

    private static bool TryGetHeader(HttpResponseMessage response, string name, out string? value)
    {
        if (response.Headers.TryGetValues(name, out var values))
        {
            value = string.Join(",", values);
            return true;
        }

        if (response.Content?.Headers.TryGetValues(name, out var contentValues) == true)
        {
            value = string.Join(",", contentValues);
            return true;
        }

        value = null;
        return false;
    }
}