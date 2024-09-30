use std::fs;

#[derive(Debug, Clone)]
struct Interval {
    start: u64,
    end: u64, // Exclusive
}

impl Interval {
    fn new(start: u64, end: u64) -> Self {
        Interval { start, end }
    }
}

struct Mapping {
    source_intervals: Vec<Interval>,
    destination_starts: Vec<u64>,
}

impl Mapping {
    fn new() -> Self {
        Mapping {
            source_intervals: Vec::new(),
            destination_starts: Vec::new(),
        }
    }

    // Add a new mapping range
    fn add_range(&mut self, destination_start: u64, source_start: u64, length: u64) {
        self.source_intervals
            .push(Interval::new(source_start, source_start + length));
        self.destination_starts.push(destination_start);
    }

    // Process input intervals against mapping's source intervals
    fn process_intervals(&self, intervals: Vec<Interval>) -> Vec<Interval> {
        let mut result_intervals = Vec::new();

        for interval in intervals {
            let mut points = Vec::new();
            // Interval boundaries
            points.push(interval.start);
            points.push(interval.end);
            // Mapped range boundaries, overlapping with interval
            for source_interval in &self.source_intervals {
                // Check for overlap
                if source_interval.end <= interval.start || source_interval.start > interval.end {
                    continue;
                }
                // Calculate overlapping range
                points.push(source_interval.start.max(interval.start));
                points.push(source_interval.end.min(interval.end));
            }
            // Remove dupes and sort
            points.sort_unstable();
            points.dedup();
            // Process sub-intervals between points
            for i in 0..points.len() - 1 {
                let sub_interval = Interval::new(points[i], points[i + 1]);
                let mut mapped = false;
                // Check against source intervals
                for (j, source_interval) in self.source_intervals.iter().enumerate() {
                    if sub_interval.start >= source_interval.start
                        && sub_interval.end <= source_interval.end
                    {
                        // If within source interval, calculate destination
                        let offset = sub_interval.start - source_interval.start;
                        let destination_start = self.destination_starts[j] + offset;
                        // Create new interval for mapped range
                        result_intervals.push(Interval::new(
                            destination_start,
                            destination_start + (sub_interval.end - sub_interval.start),
                        ));
                        mapped = true;
                        break;
                    }
                }
                if !mapped {
                    // Keep original sub-interval
                    result_intervals.push(sub_interval.clone());
                }
            }
        }
        result_intervals
    }
}

pub fn run() {
    // Parse input file into segments
    let input = fs::read_to_string("inputs/input_05.txt").expect("Failed to read file");
    let segments: Vec<&str> = input.trim().split("\n\n").collect();

    // Parse seeds into intervals
    let seeds_intervals: Vec<Interval> = segments.get(0).map_or_else(Vec::new, |segment| {
        let mut intervals = Vec::new();
        let parts: Vec<u64> = segment
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        for chunk in parts.chunks(2) {
            if chunk.len() == 2 {
                intervals.push(Interval::new(chunk[0], chunk[0] + chunk[1]));
            }
        }
        intervals
    });

    // Parse mappings
    let mut mappings: Vec<Mapping> = Vec::new();
    for segment in segments.iter().skip(1) {
        let mut mapping = Mapping::new();
        for line in segment.lines().filter(|l| !l.is_empty()) {
            if line.contains(':') {
                continue; // Skip mapping headers
            }
            let parts: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            if parts.len() == 3 {
                mapping.add_range(parts[0], parts[1], parts[2]);
            }
        }
        mappings.push(mapping);
    }

    // Process intervals through mappings
    let mut current_intervals = seeds_intervals;
    for mapping in mappings {
        current_intervals = mapping.process_intervals(current_intervals);
    }

    // Find the lowest location number
    if let Some(lowest_location) = current_intervals.iter().map(|i| i.start).min() {
        println!("Solution: {}", lowest_location);
    } else {
        println!("Solution not found");
    }
}
