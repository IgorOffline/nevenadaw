const std = @import("std");
const core = @import("core");

pub fn main() !void {
    core.log_info("<ZIG-MULTI-FRONTEND-MAIN>");
    const sum = core.math_sum(2, 3);
    core.log_info_args("Sum: {d}", .{sum});
}
