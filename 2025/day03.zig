const std = @import("std");

const batteries = @embedFile("day03.txt");
const battery_size = 12;

fn total_joltage(input: []const u8) !u64 {
    var joltage: u64 = 0;

    var battery_stack: [battery_size]u8 = undefined;
    var stack_len: usize = 0;

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        const bank = std.mem.trim(u8, line, " ");
        joltage += try max_joltage(bank, &battery_stack, &stack_len);
        stack_len = 0;
    }

    return joltage;
}

fn max_joltage(bank: []const u8, battery_stack: *[battery_size]u8, stack_len: *usize) !u64 {
    for (0..bank.len, bank) |i, battery| {
        while (
        // stack not empty
        stack_len.* > 0
            // this batter is larger than the most recent in the stack
        and battery_stack[stack_len.* - 1] < battery
            // there are enough elements to fill the rest of the battery array
        and stack_len.* + (bank.len - i) > battery_size) {
            stack_len.* -= 1;
        }
        if (stack_len.* < battery_size) {
            battery_stack[stack_len.*] = battery;
            stack_len.* += 1;
        }
    }

    var result: u64 = 0;
    for (battery_stack[0..stack_len.*]) |digit| {
        result = result * 10 + (digit - '0');
    }
    return result;
}

pub fn main() !void {
    const soln = try total_joltage(batteries);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const input =
        \\ 987654321111111
        \\ 811111111111119
        \\ 234234234234278
        \\ 818181911112111
    ;
    try std.testing.expectEqual(3121910778619, total_joltage(input));
}
