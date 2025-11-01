const sdl3 = @import("sdl3");
const std = @import("std");

const screen_width: c_int = 1280;
const screen_height: c_int = 720;

const fps = 60;

pub fn main() !void {
    //var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    //const allocator = gpa.allocator();
    //defer _ = gpa.deinit();

    const log_app = sdl3.log.Category.application;

    try sdl3.init(.{ .video = true, .events = true });
    defer sdl3.quit(.{ .video = true, .events = true });

    const window, const renderer = try sdl3.render.Renderer.initWithWindow(
        "Zig SDL3",
        screen_width,
        screen_height,
        .{ .resizable = true },
    );
    defer window.deinit();
    defer renderer.deinit();

    var frame_capper = sdl3.extras.FramerateCapper(f32){ .mode = .{ .unlimited = {} } };
    renderer.setVSync(.{ .on_each_num_refresh = 1 }) catch {
        frame_capper.mode = .{ .limited = fps };
    };

    var quit_app = false;
    while (!quit_app) {
        const dt = frame_capper.delay();
        _ = dt;

        while (sdl3.events.poll()) |event| {
            switch (event) {
                .quit, .terminating => quit_app = true,
                .key_down => |key| {
                    if (key.key == .a) {
                        // Handle the potential error from logInfo
                        log_app.logInfo("<+A>", .{}) catch {
                            std.debug.print("<-A>", .{});
                        };
                    } else if (key.key == .escape) {
                        quit_app = true;
                    }
                },
                else => {},
            }
        }

        const blue_gray_dark_primary_color = .{ .r = 69, .g = 90, .b = 100, .a = 255 };
        try renderer.setDrawColor(.{ .r = blue_gray_dark_primary_color.r, .g = blue_gray_dark_primary_color.g, .b = blue_gray_dark_primary_color.b, .a = blue_gray_dark_primary_color.a });
        try renderer.clear();
        try renderer.present();
    }
}
