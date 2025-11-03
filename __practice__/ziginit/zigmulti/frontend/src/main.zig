const std = @import("std");
const core = @import("core");

pub fn main() !void {
    core.log_info("<ZIG-MULTI-FRONTEND-MAIN>");
    const sum = core.math_sum(2, 3);
    var buffer: [1024]u8 = undefined;
    const message = std.fmt.bufPrint(&buffer, "Sum: {d}", .{sum}) catch "err-format-174e7949";
    core.log_info(message);
}
