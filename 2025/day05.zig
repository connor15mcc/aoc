const std = @import("std");

const database = @embedFile("day05.txt");

const FreshIdRange = struct {
    low: usize,
    high: usize,

    fn cmpByLow(_: void, a: FreshIdRange, b: FreshIdRange) bool {
        return a.low < b.low;
    }
};

fn count_fresh(alloc: std.mem.Allocator, input: []const u8) !u64 {
    var fresh_ranges = std.ArrayList(FreshIdRange).empty;
    defer fresh_ranges.deinit(alloc);

    var it = std.mem.splitScalar(u8, input, '\n');
    while (it.next()) |line| {
        // blank line separating fresh ranges and available ingredients
        if (line.len == 0) break;

        var rangeIt = std.mem.tokenizeScalar(u8, line, '-');
        const low = rangeIt.next().?;
        const high = rangeIt.next().?;
        std.debug.assert(rangeIt.next() == null);

        try fresh_ranges.append(alloc, .{
            .low = try std.fmt.parseInt(usize, low, 10),
            .high = try std.fmt.parseInt(usize, high, 10),
        });
    }
    std.mem.sortUnstable(FreshIdRange, fresh_ranges.items, {}, FreshIdRange.cmpByLow);

    return try numFresh(fresh_ranges.items);
}

fn numFresh(ranges: []const FreshIdRange) !u64 {
    if (ranges.len == 0) return 0;

    var count: u64 = 0;
    var low = ranges[0].low;
    var high = ranges[0].high;

    for (ranges[1..]) |range| {
        // (non-strictly) overlapping, so extend upper bound of tracked range
        if (range.low <= high + 1) {
            high = @max(high, range.high);
            continue;
        }

        // no overlap, update count and track the new range
        count += (high - low + 1);
        low = range.low;
        high = range.high;
    }

    // all prev counted + final tracked range
    return count + (high - low + 1);
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

    const soln = try count_fresh(alloc, database);
    std.debug.print("Solution: {d}\n", .{soln});
}

test "provided example" {
    const alloc = std.testing.allocator;
    const input =
        \\3-5
        \\10-14
        \\16-20
        \\12-18
        \\
        \\1
        \\5
        \\8
        \\11
        \\17
        \\32
    ;
    try std.testing.expectEqual(14, count_fresh(alloc, input));
}
