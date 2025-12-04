const std = @import("std");

const batteries = @embedFile("day03.txt");
const battery_size = 12;

fn total_joltage(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var joltage: u64 = 0;

    var battery_stack = try std.ArrayList(u8).initCapacity(alloc, battery_size);
    defer battery_stack.deinit(alloc);

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        const bank = std.mem.trim(u8, line, " ");
        joltage += try max_joltage(alloc, bank, &battery_stack);
        battery_stack.clearRetainingCapacity();
    }

    return joltage;
}

fn max_joltage(alloc: std.mem.Allocator, bank: []const u8, battery_stack: *std.ArrayList(u8)) !u64 {
    for (0..bank.len, bank) |i, battery| {
        while (
        // stack not empty
        battery_stack.items.len > 0
            // this batter is larger than the most recent in the stack
        and battery_stack.getLast() < battery
            // there are enough elements to fill the rest of the battery array
        and battery_stack.items.len + (bank.len - i) > battery_size) {
            _ = battery_stack.pop();
        }
        if (battery_stack.items.len < battery_size) {
            try battery_stack.append(alloc, battery);
        }
    }

    var result: u64 = 0;
    for (battery_stack.items) |digit| {
        result = result * 10 + (digit - '0');
    }
    return result;
}

pub fn main() !void {
    var sfa = std.heap.stackFallback(4096, std.heap.page_allocator);
    const alloc = sfa.get();

    const soln = try total_joltage(alloc, batteries);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;

    const input =
        \\ 987654321111111
        \\ 811111111111119
        \\ 234234234234278
        \\ 818181911112111
    ;
    try std.testing.expectEqual(3121910778619, total_joltage(alloc, input));
}
