const std = @import("std");

pub fn math_sum(a: i32, b: i32) i32 {
    return a + b;
}

pub fn log_info(message: ?[]const u8) void {
    const write: []const u8 = message orelse "";

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const line_with_newline = std.fmt.allocPrint(allocator, "{s}\n", .{write}) catch {
        std.fs.File.stdout().writeAll(write) catch |err| {
            std.debug.print("err-be2c4251: {s}\n", .{@errorName(err)});
        };
        return;
    };

    std.fs.File.stdout().writeAll(line_with_newline) catch |err| {
        std.debug.print("err-b4e9b1d0: {s}\n", .{@errorName(err)});
    };
}
