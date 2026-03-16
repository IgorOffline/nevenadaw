namespace RustyMarchCore;

public record RustyImage(
    string Name,
    string? ImageUrl = null,
    string? ImageWidth = null,
    string? ImageHeight = null,
    string? ImageSha384 = null);