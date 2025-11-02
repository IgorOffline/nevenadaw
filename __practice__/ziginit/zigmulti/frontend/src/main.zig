const std = @import("std");
const core = @import("core");

pub fn main() !void {
    std.debug.print("<ZIG-MULTI-FRONTEND-MAIN>\n", .{});
    const sum = core.sum(2, 3);
    std.debug.print("Sum: {d}\n", .{sum});
}
