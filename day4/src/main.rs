use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input1.txt").unwrap();
    let parsed = parse_guard_shedule(&input);
    let sleep_map = construct_sleep_map(&parsed);
    println!("{}", get_guard_code(&sleep_map));
    println!("{}", get_guard_code2(&sleep_map));
}

#[derive(Debug, Eq, PartialOrd, PartialEq, Ord)]
struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
enum LineType {
    Guard(Time, u16),
    Sleep(Time),
    Wake(Time),
}

fn parse_guard_shedule(input: &str) -> Vec<LineType> {
    let re = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] ([a-z|A-Z]+) #?(\d*)?").unwrap();
    let mut entries: Vec<_> = re.captures_iter(input).map(|cap|{
        let time = Time {
            year: cap.get(1).unwrap().as_str().parse().unwrap(),
            month: cap.get(2).unwrap().as_str().parse().unwrap(),
            day: cap.get(3).unwrap().as_str().parse().unwrap(),
            hour: cap.get(4).unwrap().as_str().parse().unwrap(),
            minute: cap.get(5).unwrap().as_str().parse().unwrap(),
        };
        match cap.get(6).unwrap().as_str() {
            "Guard" => LineType::Guard(time, cap.get(7).unwrap().as_str().parse().unwrap()),
            "falls" => LineType::Sleep(time),
            "wakes" => LineType::Wake(time),
            _ => panic!("Found funny line!"),
        }
    }).collect();
    entries.sort_by(|item1, item2| {
        let time1 = match item1 {
            LineType::Guard(time, _) => time,
            LineType::Sleep(time) => time,
            LineType::Wake(time) => time,
        };
        let time2 = match item2 {
            LineType::Guard(time, _) => time,
            LineType::Sleep(time) => time,
            LineType::Wake(time) => time,
        };
        time1.cmp(time2)
    });
    entries
}

fn construct_sleep_map(shedule: &[LineType]) -> HashMap<u16, ([u16; 60], u32)> {
    let mut sleep_map: HashMap<u16, ([u16; 60], u32)> = HashMap::new();
    let mut current_id = 0;
    let mut current_from = 0;
    for item in shedule {
        match item {
            LineType::Guard(_,  id) => {
                current_id = *id;
            } 
            LineType::Sleep(time) => {
                current_from = time.minute;
            }
            LineType::Wake(time) => {
                let entry = sleep_map.entry(current_id).or_insert(([0u16; 60], 0));
                entry.1 += (time.minute - current_from) as u32;
                for minute in current_from..time.minute {
                    entry.0[minute as usize] += 1;
                }
            }
        }
    }
    sleep_map
}

fn get_guard_code(sleep_map: &HashMap<u16, ([u16; 60], u32)>) -> u32 {
    let max_sleeper = sleep_map.iter().max_by_key(|(_, (_, k))| k).unwrap();
    let sleeper_id = max_sleeper.0;
    let sleep_minutes: [u16; 60] = (max_sleeper.1).0.clone();
    let max_minute = sleep_minutes.into_iter().enumerate().max_by_key(|&(_, k)| k).unwrap();
    *sleeper_id as u32 * max_minute.0 as u32
}

fn get_guard_code2(sleep_map: &HashMap<u16, ([u16; 60], u32)>) -> u32 {
    let max_minute_sleeper = sleep_map.iter().max_by_key(|(_, (minutes, _))| {
        minutes.iter().max().unwrap()
    }).unwrap();
    
    let sleeper_id = max_minute_sleeper.0;
    let sleep_minutes: [u16; 60] = (max_minute_sleeper.1).0.clone();
    let max_minute = sleep_minutes.into_iter().enumerate().max_by_key(|&(_, k)| k).unwrap();
    *sleeper_id as u32 * max_minute.0 as u32
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_input() {
        let input = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-03 00:29] wakes up
[1518-11-05 00:55] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep";

        let parsed = parse_guard_shedule(input);
        let sleep_map = construct_sleep_map(&parsed);
        assert_eq!(10 * 24, get_guard_code(&sleep_map));
        assert_eq!(99 * 45, get_guard_code2(&sleep_map));
    }
}
