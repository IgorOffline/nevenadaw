using System.Security.Cryptography;
using CymbalCore.Cymbal;

namespace CymbalCore.CymbalWeb;

public class CymbalWeb(CymbalRegina regina)
{
    private static readonly HttpClient Client = new();

    public CymbalRegina Get()
    {
        if (string.IsNullOrEmpty(regina.CymbalConfig?.Url))
            throw new CymbalException("Cymbal URL is missing in configuration");

        try
        {
            using var request = new HttpRequestMessage(HttpMethod.Get, regina.CymbalConfig.Url);
            using var response = Client.Send(request);
            response.EnsureSuccessStatusCode();
            return regina with { CymbalConfig = regina.CymbalConfig with { Success = true } };
        }
        catch (Exception ex) when (ex is not CymbalException)
        {
            throw new CymbalException($"Failed to get status from {regina.CymbalConfig.Url}: {ex.Message}");
        }
    }

    public CymbalAssetBytes GetAssetBytes(string? url)
    {
        if (string.IsNullOrEmpty(url)) throw new CymbalException("Asset URL is null or empty");

        try
        {
            using var request = new HttpRequestMessage(HttpMethod.Get, url);
            using var response = Client.Send(request);
            response.EnsureSuccessStatusCode();

            using var ms = new MemoryStream();
            response.Content.ReadAsStream().CopyTo(ms);

            string? serverSha384 = null;
            if (TryGetHeader(response, "x-sha384", out var value)) serverSha384 = value;

            var bytes = ms.ToArray();
            var sha384 = Convert.ToHexString(SHA384.HashData(bytes)).ToLower();
            return new CymbalAssetBytes(bytes, sha384, serverSha384);
        }
        catch (Exception ex) when (ex is not CymbalException)
        {
            throw new CymbalException($"Failed to fetch asset from {url}: {ex.Message}");
        }
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