namespace CymbalCore.Cymbal;

public record CymbalRegina(
    CymbalConfig? CymbalConfig = null,
    CymbalImageConfig? ImageBlue = null,
    CymbalImageConfig? ImageGreen = null,
    CymbalImageConfig? ImageRed = null,
    CymbalNoteConfig? NoteC3 = null,
    CymbalNoteConfig? NoteCs3 = null,
    CymbalNoteConfig? NoteD3 = null,
    CymbalNoteConfig? NoteDs3 = null,
    CymbalNoteConfig? NoteE3 = null,
    CymbalNoteConfig? NoteF3 = null);