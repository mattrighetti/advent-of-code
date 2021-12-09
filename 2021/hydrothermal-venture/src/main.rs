use std::{
    fs::File, 
    io::{
        BufReader,
        BufRead
    }, 
    path::Path, collections::HashSet
};

#[derive(Debug, Clone)]
struct Segment {
    start: Point,
    end: Point
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }

    fn from(str: &str) -> Self {
        let vals: Vec<&str> = str.split(',').collect();
        let x = vals.get(0).unwrap().trim().parse::<i32>().ok().unwrap();
        let y = vals.get(1).unwrap().trim().parse::<i32>().ok().unwrap();

        Point(x, y)
    }
}

impl Segment {
    #[allow(dead_code)]
    fn new(start: Point, end: Point) -> Self {
        Segment {
            start,
            end
        }
    }

    fn from(str: &str) -> Self {
        let vals: Vec<&str> = str.split(" -> ").collect();
        let start = Point::from(vals.get(0).unwrap());
        let end = Point::from(vals.get(1).unwrap());
        
        Segment {
            start,
            end
        }
    }
}

impl Iterator for Segment {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end == self.start {
            return None;
        }

        // Norm vector
        let mut i: i32 = 0;
        let mut j: i32 = 0;

        if self.end.0 != self.start.0 {
            i = (self.end.0 - self.start.0)/(self.end.0 - self.start.0).abs();
        }

        if self.end.1 != self.start.1 {
            j = (self.end.1 - self.start.1)/(self.end.1 - self.start.1).abs();
        }

        let nx = self.start.clone();
        self.start = Point::new(self.start.0 + i, self.start.1 + j);

        Some(nx)
    }
}

fn main() {
    let segments = parse_from_input(Path::new("input"));

    // Part one
    let horiz_vert_segments: Vec<Segment> = segments.clone().into_iter()
        .filter(|x| x.start.0 == x.end.0 || x.start.1 == x.end.1)
        .collect();
    
    let num_overlapping = get_overlapping(&horiz_vert_segments).len();
    println!("Overlapping Horiz/Vert: {}", num_overlapping);

    // Part two
    let horiz_vert_diag_segments: Vec<Segment> = segments.into_iter()
        .filter(|segment| 
            (segment.start.0 - segment.end.0).abs() == (segment.start.1 - segment.end.1).abs() ||
            segment.start.0 == segment.end.0 || 
            segment.start.1 == segment.end.1
        )
        .collect();

    let num_overlapping = get_overlapping(&horiz_vert_diag_segments).len();
    println!("Overlapping Horiz/Vert/Diag45: {}", num_overlapping);
}

fn get_overlapping(segments: &Vec<Segment>) -> Vec<Point> {
    let mut values = Vec::new();
    let mut set: HashSet<Point> = HashSet::new();

    for segment in segments {
        for point in segment.to_owned().into_iter() {
            if set.contains(&point) && !values.contains(&point) {
                values.push(point);
            } else {
                set.insert(point);
            }
        }

        if set.contains(&segment.end) && !values.contains(&segment.end) {
            values.push(segment.end.clone());
        } else {
            set.insert(segment.end.clone());
        }
    }

    values
}

fn parse_from_input(path: &Path) -> Vec<Segment> {
    let file = File::open(&path).ok().unwrap();
    let file_reader = BufReader::new(file);

    let mut segments = Vec::new();
    for line in file_reader.lines().map(|l| l.unwrap()) {
        segments.push(Segment::from(&line))
    }

    segments
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_segment_iterator() {
        let start = Point::new(9, 7);
        let end = Point::new(9, 9);
        let mut segment = Segment::new(start, end);
        assert_eq!(segment.next(), Some(Point::new(9, 7)));
        assert_eq!(segment.next(), Some(Point::new(9, 8)));
        assert_eq!(segment.next(), None);
    }
    
    #[test]
    fn test_diagonal() {
        let start = Point::new(15, 10);
        let end = Point::new(20, 15);
        let mut segment = Segment::new(start, end);
        assert_eq!(segment.next(), Some(Point::new(15, 10)));
        assert_eq!(segment.next(), Some(Point::new(16, 11)));
        assert_eq!(segment.next(), Some(Point::new(17, 12)));
        assert_eq!(segment.next(), Some(Point::new(18, 13)));
        assert_eq!(segment.next(), Some(Point::new(19, 14)));
        assert_eq!(segment.next(), None);

        let start = Point::new(10, 10);
        let end = Point::new(20, 20);
        let mut segment = Segment::new(start, end);
        assert_eq!(segment.next(), Some(Point::new(10, 10)));
        assert_eq!(segment.next(), Some(Point::new(11, 11)));
        assert_eq!(segment.next(), Some(Point::new(12, 12)));
        assert_eq!(segment.next(), Some(Point::new(13, 13)));
        assert_eq!(segment.next(), Some(Point::new(14, 14)));
        assert_eq!(segment.next(), Some(Point::new(15, 15)));
        assert_eq!(segment.next(), Some(Point::new(16, 16)));
        assert_eq!(segment.next(), Some(Point::new(17, 17)));
        assert_eq!(segment.next(), Some(Point::new(18, 18)));
        assert_eq!(segment.next(), Some(Point::new(19, 19)));
        assert_eq!(segment.next(), None);
    }
}