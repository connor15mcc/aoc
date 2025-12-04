const std = @import("std");

const positions = @embedFile("day04.txt");

const dirs = [_][2]i32{
    .{ -1, -1 }, .{ -1, 0 }, .{ -1, 1 },
    .{ 0, -1 },  .{ 0, 1 },  .{ 1, -1 },
    .{ 1, 0 },   .{ 1, 1 },
};

fn accessible_paper(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var grid = std.ArrayList([]const u8).empty;
    defer grid.deinit(alloc);

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        try grid.append(alloc, line);
    }

    var accessible: u64 = 0;

    const rows = grid.items.len;
    std.debug.assert(rows > 0);
    const cols = grid.items[0].len;

    for (0.., grid.items) |i, row| {
        std.debug.print("row: {s}\n", .{row});
        for (0.., row) |j, cell| {
            if (cell != '@') continue;

            var num_neighbors: usize = 0;
            for (dirs) |dir| {
                const ni = @as(i32, @intCast(i)) + dir[0];
                const nj = @as(i32, @intCast(j)) + dir[1];

                if (ni < 0 or ni >= rows or nj < 0 or nj >= cols) continue;

                const ni_usize = @as(usize, @intCast(ni));
                const nj_usize = @as(usize, @intCast(nj));
                if (grid.items[ni_usize][nj_usize] == '@') {
                    num_neighbors += 1;
                }
            }

            if (num_neighbors < 4) {
                accessible += 1;
            }
        }
    }
    return accessible;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) {
            @panic("Memory leak detected!");
        }
    }
    const alloc = gpa.allocator();

    const soln = try accessible_paper(alloc, positions);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input =
        \\..@@.@@@@.
        \\@@@.@.@.@@
        \\@@@@@.@.@@
        \\@.@@@@..@.
        \\@@.@@@@.@@
        \\.@@@@@@@.@
        \\.@.@.@.@@@
        \\@.@@@.@@@@
        \\.@@@@@@@@.
        \\@.@.@@@.@.
    ;
    try std.testing.expectEqual(13, accessible_paper(alloc, input));
}
