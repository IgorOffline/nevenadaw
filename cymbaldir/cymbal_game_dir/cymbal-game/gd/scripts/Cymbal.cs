using System;
using System.Text;
using CymbalCore.Cymbal;
using CymbalCore.CymbalWeb;
using Godot;

public partial class Cymbal : Node2D
{
	private AudioStreamPlayer? _audioPlayer;
	private CymbalLogger? _logger;
	private CymbalRegina? _regina;
	private CymbalWeb? _web;

	public override void _Ready()
	{
		_logger = new CymbalLogger();
		_regina = new CymbalConfigLoader().Get();
		_web = new CymbalWeb(_regina);
		var regina = _web.Get();
		_logger.Print($"[Cymbal 0.1.0] Regina Success: {regina.CymbalConfig!.Success}");

		if (regina.ImageBlue != null) CreateImageNode(regina.ImageBlue, new Vector2(80, 60), "Blue");
		if (regina.ImageGreen != null) CreateImageNode(regina.ImageGreen, new Vector2(150, 60), "Green");
		if (regina.ImageRed != null) CreateImageNode(regina.ImageRed, new Vector2(220, 60), "Red");

		_audioPlayer = new AudioStreamPlayer();
		AddChild(_audioPlayer);

		CreateNotePlayingNode("C3 Note", "Playing C3 Note", new Vector2(50, 150), new Vector2(120, 60));

		var icon = GetNodeOrNull<Sprite2D>("Icon");
		if (icon != null)
			_logger.Print($"Icon found at: {icon.Position}");
		else
			_logger.PrintErr("Icon node not found!");
	}

	private void CreateImageNode(CymbalImageConfig config, Vector2 position, string name)
	{
		var (bytes, sha384) = _web!.GetAssetBytes($"{_regina!.CymbalConfig!.Url!}{config.ImageUrl}");

		if (!string.IsNullOrEmpty(config.ImageSha384) && !string.IsNullOrEmpty(sha384))
		{
			if (!string.Equals(config.ImageSha384, sha384, StringComparison.OrdinalIgnoreCase))
				throw new CymbalException($"SHA384 mismatch for {name}: {config.ImageSha384} != {sha384}");

			_logger!.Print($"SHA384 verified for {name}: {sha384}");
		}

		var image = new Image();
		var error = image.LoadPngFromBuffer(bytes);
		if (error != Error.Ok) throw new CymbalException($"Failed to load PNG for {name} from buffer: {error}");
		image.Resize(60, 60);
		var texture = ImageTexture.CreateFromImage(image);
		var sprite = new Sprite2D();
		sprite.Texture = texture;
		sprite.Position = position;
		sprite.Name = $"DynamicImage_{name}";
		AddChild(sprite);

		_logger!.Print($"Created PNG Image Node '{name}' at {sprite.Position}");
	}

	private void CreateNotePlayingNode(string text, string logMessage, Vector2 position, Vector2 size)
	{
		var button = new Button();
		button.Text = text;
		button.Position = position;
		button.Size = size;
		button.Pressed += () =>
		{
			if (_regina?.NoteC3 != null)
			{
				var (bytes, sha384) = _web!.GetAssetBytes($"{_regina!.CymbalConfig!.Url!}{_regina.NoteC3.NoteUrl}");
				if (!string.IsNullOrEmpty(_regina.NoteC3.NoteSha384) && !string.IsNullOrEmpty(sha384))
				{
					if (string.Equals(_regina.NoteC3.NoteSha384, sha384, StringComparison.OrdinalIgnoreCase))
					{
						_logger!.Print($"SHA384 verified for Note: {sha384}");
						_logger!.Print(logMessage);
						PlaySound(bytes);
					}
					else
					{
						_logger!.PrintErr($"SHA384 mismatch for Note: {_regina.NoteC3.NoteSha384} != {sha384}");
					}
				}
				else
				{
					_logger!.Print(logMessage);
					PlaySound(bytes);
				}
			}
			else
			{
				_logger!.Print(logMessage);
			}
		};
		button.Name = $"DynamicButton_{text}";
		AddChild(button);

		_logger!.Print($"Created Button Node '{text}' at {button.Position} with size {button.Size}");
	}

	private void CreateButtonNode(string text, string logMessage, Vector2 position, Vector2 size)
	{
		var button = new Button();
		button.Text = text;
		button.Position = position;
		button.Size = size;
		button.Pressed += () => _logger!.Print(logMessage);
		button.Name = $"DynamicButton_{text}";
		AddChild(button);

		_logger!.Print($"Created Button Node '{text}' at {button.Position} with size {button.Size}");
	}

	private void PlaySound(byte[] wavBytes)
	{
		if (_audioPlayer == null || wavBytes.Length < 44) return;

		try
		{
			if (Encoding.ASCII.GetString(wavBytes, 0, 4) != "RIFF" ||
				Encoding.ASCII.GetString(wavBytes, 8, 4) != "WAVE")
				return;

			short channels = 0;
			int sampleRate = 0;
			short bitsPerSample = 0;
			byte[]? data = null;

			var offset = 12;
			while (offset + 8 < wavBytes.Length)
			{
				var chunkId = Encoding.ASCII.GetString(wavBytes, offset, 4);
				var chunkSize = BitConverter.ToInt32(wavBytes, offset + 4);
				var chunkDataOffset = offset + 8;

				if (chunkId == "fmt ")
				{
					channels = BitConverter.ToInt16(wavBytes, chunkDataOffset + 2);
					sampleRate = BitConverter.ToInt32(wavBytes, chunkDataOffset + 4);
					bitsPerSample = BitConverter.ToInt16(wavBytes, chunkDataOffset + 14);
				}
				else if (chunkId == "data")
				{
					data = new byte[chunkSize];
					Array.Copy(wavBytes, chunkDataOffset, data, 0, chunkSize);
				}

				offset += 8 + (chunkSize % 2 == 0 ? chunkSize : chunkSize + 1);
			}

			if (data == null || channels == 0 || sampleRate == 0 || bitsPerSample == 0) return;

			var stream = new AudioStreamWav();
			stream.MixRate = sampleRate;
			stream.Stereo = channels == 2;

			if (bitsPerSample == 24)
			{
				var sampleCount = data.Length / 3;
				var pcm16 = new byte[sampleCount * 2];
				for (int i = 0; i < sampleCount; i++)
				{
					pcm16[i * 2] = data[i * 3 + 1];
					pcm16[i * 2 + 1] = data[i * 3 + 2];
				}

				stream.Data = pcm16;
				stream.Format = AudioStreamWav.FormatEnum.Format16Bits;
			}
			else if (bitsPerSample == 16)
			{
				stream.Data = data;
				stream.Format = AudioStreamWav.FormatEnum.Format16Bits;
			}
			else if (bitsPerSample == 8)
			{
				stream.Data = data;
				stream.Format = AudioStreamWav.FormatEnum.Format8Bits;
			}
			else
			{
				_logger!.PrintErr($"Unsupported bit depth: {bitsPerSample}");
				return;
			}

			_audioPlayer.Stream = stream;
			_audioPlayer.Play();
		}
		catch (Exception ex)
		{
			_logger!.PrintErr($"Failed to play sound: {ex.Message}");
		}
	}

	public override void _Process(double delta)
	{
	}
}
