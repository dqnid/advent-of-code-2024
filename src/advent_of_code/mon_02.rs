use utils::calc_distance;

use super::*;

pub fn check_reports_safety(input: &str) -> ReportSafety {
    let report_list: [[Report; 5]; 6] = [
        [7, 6, 4, 2, 1],
        [1, 2, 7, 8, 9],
        [9, 7, 6, 2, 1],
        [1, 3, 2, 4, 5],
        [8, 6, 4, 4, 1],
        [1, 3, 6, 7, 9],
    ];

    let mut safe_count: ReportSafety = 0;

    // 1. All levels must be all increasing or decreasing
    // 2. All levels must change by at leat one and at most three

    for report in report_list {
        if report.len() < 2 {
            safe_count += 1;
            break;
        }

        let mut safe = true;
        let initial_direction: ReportDirection = get_report_direction(&report[0..=1]);

        'report_check: for index in 1..report.len() {
            let prev = index - 1;
            let distance = calc_distance(report[index], report[prev]);
            let direction = get_report_direction(&[report[prev], report[index]]);

            if report[index] == report[prev]
                || distance < 1
                || distance > 3
                || direction != initial_direction
            {
                safe = false;
                break 'report_check;
            }
        }

        if safe {
            safe_count += 1;
        }
    }

    safe_count
}

// FIXME: this is not a good function, since may try to access an invalid index
fn get_report_direction(report: &[Report]) -> ReportDirection {
    if report[1] - report[0] > 0 {
        ReportDirection::Increasing
    } else {
        ReportDirection::Decreasing
    }
}
