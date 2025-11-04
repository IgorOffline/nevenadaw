const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var client = std.http.Client{ .allocator = allocator };
    defer client.deinit();

    var response_buffer = try std.ArrayList(u8).initCapacity(allocator, 1024);
    defer response_buffer.deinit(allocator);

    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    std.debug.print("Making request...\n", .{});

    const response = try client.fetch(.{
        .location = .{ .url = "http://localhost:8080/" },
        .response_writer = stdout,
    });

    const status = response.status;
    std.debug.print("String: {s}\n", .{status.phrase() orelse ""});

    try stdout.flush();
}
