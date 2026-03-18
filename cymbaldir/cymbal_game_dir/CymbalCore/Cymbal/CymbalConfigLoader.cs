using Tomlyn;
using Tomlyn.Model;

namespace CymbalCore.Cymbal;

public class CymbalConfigLoader
{
    public CymbalRegina Get()
    {
        const string configPath = @"C:\Users\igor\dev\nevenadawdir\nevenadaw\cymbaldir\cymbal.toml";

        if (!File.Exists(configPath)) throw new CymbalException($"Config file not found at: {configPath}");

        try
        {
            var content = File.ReadAllText(configPath);
            var model = TomlSerializer.Deserialize<TomlTable>(content);
            if (model == null || !model.TryGetValue("regina", out var sectionObj) ||
                sectionObj is not TomlTable section)
                throw new CymbalException("Missing [regina] section in config file");

            string? url = null;
            if (section.TryGetValue("url", out var urlObj) && urlObj is string urlValue) url = urlValue;

            var imageBlue = ParseImage(section, "imageblue");
            var imageGreen = ParseImage(section, "imagegreen");
            var imageRed = ParseImage(section, "imagered");
            var noteC3 = ParseNote(section, "notec3");
            var noteCs3 = ParseNote(section, "notecs3");
            var noteD3 = ParseNote(section, "noted3");
            var noteDs3 = ParseNote(section, "noteds3");
            var noteE3 = ParseNote(section, "notee3");
            var noteF3 = ParseNote(section, "notef3");

            return new CymbalRegina(new CymbalConfig(url, null), imageBlue, imageGreen, imageRed, noteC3, noteCs3,
                noteD3, noteDs3, noteE3, noteF3);
        }
        catch (Exception ex) when (ex is not CymbalException)
        {
            throw new CymbalException($"Failed to load config: {ex.Message}");
        }
    }

    private static CymbalImageConfig? ParseImage(TomlTable parent, string key)
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
            return new CymbalImageConfig(imageUrl, imageWidth, imageHeight, sampleWidth, sampleHeight, sha384);
        }

        return null;
    }

    private static CymbalNoteConfig? ParseNote(TomlTable parent, string key)
    {
        if (parent.TryGetValue(key, out var obj) && obj is TomlTable table)
        {
            string? noteUrl = null, sha384 = null;
            if (table.TryGetValue("note_url", out var nu) && nu is string nuv) noteUrl = nuv;
            if (table.TryGetValue("note_sha384", out var s) && s is string sv) sha384 = sv;
            return new CymbalNoteConfig(noteUrl, sha384);
        }

        return null;
    }
}