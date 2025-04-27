const std = @import("std");

pub fn hello() void {
    std.debug.print("Hello from {s}\n", .{"snake.zig"});
}
