use std::{
    env,
    io::{self, Read},
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_str_to_i32(str: &str) -> Vec<i32> {
    let mut parsed_str = vec![];

    for word in str.split_whitespace() {
        parsed_str.push(word.parse().unwrap());
    }
    parsed_str
}

fn get_skyscraper_rules() -> Result<Vec<i32>, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        Ok(parse_str_to_i32(&args[1]))
    } else {
        Err("Invalid number of arguments".to_string())
    }
}

fn get_skyscraper_grid() -> Vec<i32> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_str_to_i32(&buf)
}

fn check_rules(skyscraper_rules: &[i32]) -> Result<(), String> {
    if skyscraper_rules.len() % 4 == 0 {
        Ok(())
    } else {
        Err("invalid rules".to_string())
    }
}

fn get_skyscraper_line(
    direction: &Direction,
    index: usize,
    grid_len: usize,
    skyscraper_grid: &[i32],
) -> Vec<i32> {
    let mut skyscraper_line = vec![];

    match direction {
        Direction::Up => {
            for (i, skyscraper) in skyscraper_grid.iter().enumerate() {
                if i % grid_len == index {
                    skyscraper_line.push(*skyscraper);
                }
            }
        }
        Direction::Down => {
            for (i, skyscraper) in skyscraper_grid.iter().enumerate().rev() {
                if i % grid_len == index {
                    skyscraper_line.push(*skyscraper);
                }
            }
        }
        Direction::Left => {
            for (i, skyscraper) in skyscraper_grid.iter().enumerate() {
                if i / grid_len == index {
                    skyscraper_line.push(*skyscraper);
                }
            }
        }
        Direction::Right => {
            for (i, skyscraper) in skyscraper_grid.iter().enumerate().rev() {
                if i / grid_len == index {
                    skyscraper_line.push(*skyscraper);
                }
            }
        }
    }
    skyscraper_line
}

fn validate_line(skyscraper_line: &[i32]) -> Result<(), String> {
    let mut copy_line = Vec::from(skyscraper_line);

    copy_line.dedup();
    if copy_line.len() != skyscraper_line.len() {
        return Err("Skyscraper line contain duplicate value".to_string());
    }

    let line_len = i32::try_from(skyscraper_line.len());

    match line_len {
        Ok(line_len) => {
            for skyscraper in skyscraper_line {
                if *skyscraper <= 0 || *skyscraper > line_len {
                    return Err("Invalid skyscraper value".to_string());
                }
            }
            Ok(())
        }
        Err(_) => Err("Invalid line size".to_string()),
    }
}

fn check_line(rule: i32, skyscraper_line: &[i32]) -> Result<(), String> {
    let mut count_skyscraper = 0;
    let mut max_skyscraper_size = 0;

    for skyscraper_size in skyscraper_line {
        if skyscraper_size > &max_skyscraper_size {
            max_skyscraper_size = *skyscraper_size;
            count_skyscraper += 1;
        }
    }
    if count_skyscraper == rule {
        Ok(())
    } else {
        Err(format!(
            "Invalid Line: rule={rule}, count={count_skyscraper}, line={skyscraper_line:?}"
        ))
    }
}

fn choice_skyscraper_line(
    i: usize,
    grid_len: usize,
    skyscraper_grid: &[i32],
) -> Result<Vec<i32>, String> {
    if i < grid_len {
        Ok(get_skyscraper_line(
            &Direction::Up,
            i % grid_len,
            grid_len,
            skyscraper_grid,
        ))
    } else if i < 2 * grid_len {
        Ok(get_skyscraper_line(
            &Direction::Down,
            (i - grid_len) % grid_len,
            grid_len,
            skyscraper_grid,
        ))
    } else if i < 3 * grid_len {
        Ok(get_skyscraper_line(
            &Direction::Left,
            (i - 2 * grid_len) % grid_len,
            grid_len,
            skyscraper_grid,
        ))
    } else if i < 4 * grid_len {
        Ok(get_skyscraper_line(
            &Direction::Right,
            (i - 3 * grid_len) % grid_len,
            grid_len,
            skyscraper_grid,
        ))
    } else {
        Err("Internal Error: invalid index".to_string())
    }
}

fn check_grid(skyscraper_rules: &[i32], skyscraper_grid: &[i32]) -> Result<(), String> {
    let grid_len = skyscraper_rules.len() / 4;

    if grid_len * grid_len != skyscraper_grid.len() {
        return Err("invalid grid".to_string());
    }
    for (i, rule) in skyscraper_rules.iter().enumerate() {
        let line = choice_skyscraper_line(i, grid_len, skyscraper_grid)?;
        validate_line(&line)?;
        check_line(*rule, &line)?;
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let skyscraper_rules = get_skyscraper_rules()?;

    check_rules(&skyscraper_rules)?;

    let skyscraper_grid = get_skyscraper_grid();

    check_grid(&skyscraper_rules, &skyscraper_grid)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_str_to_i32() {
        assert_eq!(parse_str_to_i32(&String::from("1 2 3")), vec![1, 2, 3]);
        assert_eq!(parse_str_to_i32(&String::from("")), vec![]);
    }

    #[test]
    fn test_check_rules() {
        assert!(check_rules(&vec![1, 1, 1, 1]).is_ok());
        assert!(check_rules(&vec![2, 1, 1, 2, 2, 1, 1, 2]).is_ok());

        assert!(check_rules(&vec![1]).is_err());
        assert!(check_rules(&vec![1, 2, 3, 4, 5]).is_err());
    }

    #[test]
    fn test_get_skyscraper_line_one_square() {
        assert_eq!(get_skyscraper_line(&Direction::Up, 0, 1, &vec![1]), vec![1]);
        assert_eq!(
            get_skyscraper_line(&Direction::Down, 0, 1, &vec![1]),
            vec![1]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Left, 0, 1, &vec![1]),
            vec![1]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Right, 0, 1, &vec![1]),
            vec![1]
        );
    }

    #[test]
    fn test_get_skyscraper_line_two_square() {
        let skyscraper_grid = vec![1, 2, 3, 4];

        assert_eq!(
            get_skyscraper_line(&Direction::Up, 0, 2, &skyscraper_grid),
            vec![1, 3]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Down, 0, 2, &skyscraper_grid),
            vec![3, 1]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Left, 0, 2, &skyscraper_grid),
            vec![1, 2]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Right, 0, 2, &skyscraper_grid),
            vec![2, 1]
        );

        assert_eq!(
            get_skyscraper_line(&Direction::Up, 1, 2, &skyscraper_grid),
            vec![2, 4]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Down, 1, 2, &skyscraper_grid),
            vec![4, 2]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Left, 1, 2, &skyscraper_grid),
            vec![3, 4]
        );
        assert_eq!(
            get_skyscraper_line(&Direction::Right, 1, 2, &skyscraper_grid),
            vec![4, 3]
        );
    }

    #[test]
    fn test_check_line() {
        assert!(check_line(1, &vec![1]).is_ok());
        assert!(check_line(2, &vec![1]).is_err());

        assert!(check_line(1, &vec![4, 3, 2, 1]).is_ok());
        assert!(check_line(2, &vec![3, 4, 2, 1]).is_ok());
        assert!(check_line(3, &vec![2, 3, 4, 1]).is_ok());
        assert!(check_line(4, &vec![1, 2, 3, 4]).is_ok());

        assert!(check_line(4, &vec![4, 3, 2, 1]).is_err());
        assert!(check_line(3, &vec![3, 4, 2, 1]).is_err());
        assert!(check_line(2, &vec![2, 3, 4, 1]).is_err());
        assert!(check_line(1, &vec![1, 2, 3, 4]).is_err());
    }

    #[test]
    fn test_choice_skyscraper_line() {
        let grid = vec![1];

        assert_eq!(choice_skyscraper_line(0, 1, &grid), Ok(vec![1]));
        assert_eq!(choice_skyscraper_line(1, 1, &grid), Ok(vec![1]));
        assert_eq!(choice_skyscraper_line(2, 1, &grid), Ok(vec![1]));
        assert_eq!(choice_skyscraper_line(3, 1, &grid), Ok(vec![1]));
        assert!(choice_skyscraper_line(4, 1, &grid).is_err());

        let grid = vec![1, 2, 2, 1];

        assert_eq!(choice_skyscraper_line(0, 2, &grid), Ok(vec![1, 2]));
        assert_eq!(choice_skyscraper_line(1, 2, &grid), Ok(vec![2, 1]));
        assert_eq!(choice_skyscraper_line(2, 2, &grid), Ok(vec![2, 1]));
        assert_eq!(choice_skyscraper_line(3, 2, &grid), Ok(vec![1, 2]));
        assert_eq!(choice_skyscraper_line(4, 2, &grid), Ok(vec![1, 2]));
        assert_eq!(choice_skyscraper_line(5, 2, &grid), Ok(vec![2, 1]));
        assert_eq!(choice_skyscraper_line(6, 2, &grid), Ok(vec![2, 1]));
        assert_eq!(choice_skyscraper_line(7, 2, &grid), Ok(vec![1, 2]));
        assert!(choice_skyscraper_line(8, 2, &grid).is_err());

        let grid = vec![
            1, 3, 5, 4, 2, 3, 5, 4, 2, 1, 2, 4, 1, 5, 3, 5, 2, 3, 1, 4, 4, 1, 2, 3, 5,
        ];

        assert_eq!(choice_skyscraper_line(0, 5, &grid), Ok(vec![1, 3, 2, 5, 4]));
    }

    #[test]
    fn test_check_grid() {
        let rules = vec![1, 1, 1, 1];
        let grid = vec![1];

        assert!(check_grid(&rules, &grid).is_ok());

        let rules = vec![2, 1, 1, 2, 2, 1, 1, 2];
        let grid = vec![1, 2, 2, 1];

        assert!(check_grid(&rules, &grid).is_ok());

        let rules = vec![3, 2, 1, 2, 4, 2, 4, 4, 2, 1, 3, 2, 3, 1, 2, 3, 4, 2, 2, 1];
        let grid = vec![
            1, 3, 5, 4, 2, 3, 5, 4, 2, 1, 2, 4, 1, 5, 3, 5, 2, 3, 1, 4, 4, 1, 2, 3, 5,
        ];

        assert!(check_grid(&rules, &grid).is_ok());

        let rules = vec![1, 2, 3, 4];
        let grid = vec![1];

        assert!(check_grid(&rules, &grid).is_err());

        let rules = vec![1, 1, 1, 1];
        let grid = vec![2];

        assert!(check_grid(&rules, &grid).is_err());

        let rules = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let grid = vec![2, 2, 2, 2];

        assert!(check_grid(&rules, &grid).is_err());
    }
}
