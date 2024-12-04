use utils::{calc_distance, read_report_list};

use super::*;

pub fn check_reports_safety(input: &str) -> ReportSafety {
    let report_list = read_report_list(input);

    let mut safe_count: ReportSafety = 0;

    // 1. All levels must be all increasing or decreasing
    // 2. All levels must change by at leat one and at most three
    for report in report_list {
        if report.len() < 2 {
            break;
        }

        if is_report_safe(&report, true) {
            safe_count += 1;
        }
    }

    safe_count
}

fn is_report_safe(report: &Report, apply_problem_dampener: bool) -> bool {
    if report.len() < 2 {
        return false;
    }
    let mut safe = true;
    let initial_direction: ReportDirection = get_report_direction(&report[0..=1]);

    'report_check: for index in 1..report.len() {
        if !is_level_pair_valid(&report[index - 1], &report[index], &initial_direction) {
            if apply_problem_dampener {
                let sliced_index_report =
                    [&report[0..index], &report[index + 1..report.len()]].concat();
                if is_report_safe(&sliced_index_report, false) {
                    break 'report_check;
                } else {
                    let sliced_prev_report =
                        [&report[0..index - 1], &report[index..report.len()]].concat();
                    if is_report_safe(&sliced_prev_report, false) {
                        break 'report_check;
                    }
                }
            }
            safe = false;
            break 'report_check;
        }
    }

    safe
}

fn is_level_pair_valid(first: &Level, second: &Level, initial_direction: &ReportDirection) -> bool {
    let distance = calc_distance(*second, *first);
    let direction = get_report_direction(&[*first, *second]);

    !(first == second || distance < 1 || distance > 3 || direction != *initial_direction)
}

// FIXME: this is not a good function, since may try to access an invalid index
fn get_report_direction(report: &[Level]) -> ReportDirection {
    if report[1] - report[0] > 0 {
        ReportDirection::Increasing
    } else {
        ReportDirection::Decreasing
    }
}
