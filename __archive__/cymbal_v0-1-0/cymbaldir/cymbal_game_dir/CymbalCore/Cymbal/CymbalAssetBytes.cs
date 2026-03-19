namespace CymbalCore.Cymbal;

public record CymbalAssetBytes(byte[] Bytes, string? Sha384, string? ServerSha384 = null);