const std = @import("std");

const snake = @import("snake");
const vaxis = @import("vaxis");
const Cell = vaxis.Cell;
const TextInput = vaxis.widgets.TextInput;
const border = vaxis.widgets.border;

const Event = union(enum) {
    key_press: vaxis.Key,
    winsize: vaxis.Winsize,
    focus_in,
    foo: u8,
};

pub fn main() !void {
    snake.hello();

    // Initialize allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) {
            std.log.err("Memory Leak", .{});
        }
    }
    const alloc = gpa.allocator();

    // Initialize a tty
    var tty = try vaxis.Tty.init();
    defer tty.deinit();

    // Initialize vaxis
    var vx = try vaxis.init(alloc, .{});
    defer vx.deinit(alloc, tty.anyWriter());
    var loop: vaxis.Loop(Event) = .{
        .tty = &tty,
        .vaxis = &vx,
    };
    try loop.init();
    try loop.start();
    defer loop.stop();

    try vx.enterAltScreen(tty.anyWriter());
    var color_idx: u8 = 0;

    var text_input = TextInput.init(alloc, &vx.unicode);
    defer text_input.deinit();

    try vx.queryTerminal(tty.anyWriter(), 1 * std.time.ns_per_s);

    while (true) {
        const event = loop.nextEvent();
        switch (event) {
            .key_press => |key| {
                color_idx = switch (color_idx) {
                    255 => 0,
                    else => color_idx + 1,
                };
                if (key.matches('c', .{.ctrl = true})) {
                    break;
                } else if (key.matches('l', .{.ctrl = true})) {
                    vx.queueRefresh();
                } else {
                    try text_input.update(.{ .key_press = key});
                }
            },
            .winsize => |ws| try vx.resize(alloc, tty.anyWriter(), ws),
            else => {},
        }
        const win = vx.window();
        win.clear();

        const style: vaxis.Style = . {
            .fg = .{ .index = color_idx},
        };

        const child = win.child(.{
            .x_off = win.width / 2 - 20,
            .y_off = win.height / 2-3,
            .width = 40,
            .height = 3,
            .border = .{
                .where = .all,
                .style = style,
            }
        });

        text_input.draw(child);
        try vx.render(tty.anyWriter());
    }

}

test "use other module" {
    try std.testing.expectEqual(@as(i32, 150), .add(100, 50));
}

