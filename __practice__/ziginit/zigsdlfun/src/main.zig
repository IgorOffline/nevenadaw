const sdl3 = @import("sdl3");
const std = @import("std");
const builtin = @import("builtin");

comptime {
    _ = sdl3.main_callbacks;
}

const fps = 60;
const window_width = 640;
const window_height = 480;

pub const _start = void;
pub const WinMainCRTStartup = void;

const allocator = if (builtin.os.tag != .emscripten) std.heap.smp_allocator else std.heap.c_allocator;

const log_app = sdl3.log.Category.application;

const AppState = struct {
    frame_capper: sdl3.extras.FramerateCapper(f32),
    window: sdl3.video.Window,
    renderer: sdl3.render.Renderer,
};

pub fn init(
    app_state: *?*AppState,
    args: [][*:0]u8,
) !sdl3.AppResult {
    _ = args;

    sdl3.errors.error_callback = &sdl3.extras.sdlErrZigLog;
    sdl3.log.setAllPriorities(.info);
    sdl3.log.setLogOutputFunction(void, &sdl3.extras.sdlLogZigLog, null);

    try log_app.logInfo("<START>", .{});

    const state = try allocator.create(AppState);
    errdefer allocator.destroy(state);

    const window, const renderer = try sdl3.render.Renderer.initWithWindow(
        "Zig SDL3",
        window_width,
        window_height,
        .{},
    );
    errdefer renderer.deinit();
    errdefer window.deinit();
    var frame_capper = sdl3.extras.FramerateCapper(f32){ .mode = .{ .unlimited = {} } };
    renderer.setVSync(.{ .on_each_num_refresh = 1 }) catch {
        frame_capper.mode = .{ .limited = fps };
    };

    const dummy: ?sdl3.video.Window = sdl3.video.Window.fromId(99999) catch null;
    _ = dummy;

    state.* = .{
        .frame_capper = frame_capper,
        .window = window,
        .renderer = renderer,
    };
    app_state.* = state;

    try log_app.logInfo("<INIT>", .{});
    return .run;
}

pub fn iterate(
    app_state: *AppState,
) !sdl3.AppResult {
    const dt = app_state.frame_capper.delay();
    _ = dt;

    const blue_gray_dark_primary_color = .{ .r = 69, .g = 90, .b = 100, .a = 255 };
    try app_state.renderer.setDrawColor(.{
        .r = blue_gray_dark_primary_color.r,
        .g = blue_gray_dark_primary_color.g,
        .b = blue_gray_dark_primary_color.b,
        .a = blue_gray_dark_primary_color.a,
    });
    try app_state.renderer.clear();
    const yellow_accent_color = .{ .r = 255, .g = 235, .b = 59, .a = 255 };
    try app_state.renderer.setDrawColor(.{
        .r = yellow_accent_color.r,
        .g = yellow_accent_color.g,
        .b = yellow_accent_color.b,
        .a = yellow_accent_color.a,
    });

    var fps_text_buf: [32]u8 = undefined;
    const fps_text = std.fmt.bufPrintZ(&fps_text_buf, "FPS: {d}", .{app_state.frame_capper.getObservedFps()}) catch "[ERR]";
    try app_state.renderer.renderDebugText(.{ .x = 0, .y = 0 }, fps_text);

    try app_state.renderer.present();
    return .run;
}

pub fn event(
    app_state: *AppState,
    curr_event: sdl3.events.Event,
) !sdl3.AppResult {
    _ = app_state;
    switch (curr_event) {
        .terminating => return .success,
        .quit => return .success,
        else => {},
    }
    return .run;
}

pub fn quit(
    app_state: ?*AppState,
    result: sdl3.AppResult,
) void {
    _ = result;
    if (app_state) |val| {
        val.renderer.deinit();
        val.window.deinit();
        allocator.destroy(val);
    }
}
