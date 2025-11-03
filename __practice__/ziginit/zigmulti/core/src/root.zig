const std = @import("std");

pub fn math_sum(a: i32, b: i32) i32 {
    return a + b;
}

pub fn log_info(message: ?[]const u8) void {
    const write: []const u8 = message orelse "";
    std.debug.print("{s}\n", .{write});
}

pub fn log_info_args(comptime format: []const u8, args: anytype) void {
    var buffer: [1024]u8 = undefined;
    const message = std.fmt.bufPrint(&buffer, format, args) catch format;

    std.fs.File.stdout().writeAll(message) catch |err| {
        std.debug.print("err-9ff96916: {s}\n", .{@errorName(err)});
    };
    std.fs.File.stdout().writeAll("\n") catch |err| {
        std.debug.print("err-d03e7948: {s}\n", .{@errorName(err)});
    };
}
