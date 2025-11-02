const std = @import("std");

pub fn sum(a: i32, b: i32) i32 {
    std.debug.print("Core: Calculating {d} + {d}\n", .{ a, b });
    return a + b;
}
