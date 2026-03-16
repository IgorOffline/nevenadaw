using Tomlyn;
using Tomlyn.Model;

namespace RustyMarchCore;

public class RustyWeb
{
    private static readonly HttpClient Client = new();

    public RustyConfig Get()
    {
        string? url = null;
        RustyImageConfig? imageBlue = null;
        RustyImageConfig? imageGreen = null;
        RustyImageConfig? imageRed = null;
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
                    if (section.TryGetValue("url", out var urlObj) && urlObj is string urlValue) url = urlValue;

                    imageBlue = ParseImage(section, "imageblue");
                    imageGreen = ParseImage(section, "imagegreen");
                    imageRed = ParseImage(section, "imagered");
                }
            }
            else
            {
                return new RustyConfig(null, null);
            }

            if (string.IsNullOrEmpty(url))
                return new RustyConfig(url, null, new RustyRegina(imageBlue, imageGreen, imageRed));

            using var request = new HttpRequestMessage(HttpMethod.Get, url);
            using var response = Client.Send(request);
            success = response.IsSuccessStatusCode;
        }
        catch
        {
            success = false;
        }

        return new RustyConfig(url, success, new RustyRegina(imageBlue, imageGreen, imageRed));
    }

    private static RustyImageConfig? ParseImage(TomlTable parent, string key)
    {
        if (parent.TryGetValue(key, out var obj) && obj is TomlTable table)
        {
            string? imageUrl = null,
                imageWidth = null,
                imageHeight = null,
                sampleWidth = null,
                sampleHeight = null,
                sha384 = null;
            if (table.TryGetValue("image_url", out var iu) && iu is string iuv) imageUrl = iuv;
            if (table.TryGetValue("image_width", out var iw) && iw is string iwv) imageWidth = iwv;
            if (table.TryGetValue("image_height", out var ih) && ih is string ihv) imageHeight = ihv;
            if (table.TryGetValue("image_sample_width", out var sw) && sw is string swv) sampleWidth = swv;
            if (table.TryGetValue("image_sample_height", out var sh) && sh is string shv) sampleHeight = shv;
            if (table.TryGetValue("image_sha384", out var s) && s is string sv) sha384 = sv;
            return new RustyImageConfig(imageUrl, imageWidth, imageHeight, sampleWidth, sampleHeight, sha384);
        }

        return null;
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