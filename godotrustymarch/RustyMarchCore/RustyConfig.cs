namespace RustyMarchCore;

public record RustyConfig(
    string? Url,
    string? ImageUrl,
    string? ImageWidth,
    string? ImageHeight,
    string? ImageSha384,
    bool? Success);