use super::*;

pub fn garden_groups(input: &str) {
    let garden = utils::read_garden_arrangement_input(input);
}

pub fn get_plot_list(map: &Garden) -> Plot {
    let plot = Plot{}
}

pub fn which_plot_contains_coords(list: &PlotList, coord: Coorinate) -> Option<usize> {
    for (plot_index, plot) in list.iter().enumerate() {
        if plot.plant_list.contains(coord) {
            return Some(plot_index);
        }
    }

    None
}

// Perimeter is calculated looping over the elements and adding every non-member.
//
// mutate the garden on every iteration to avoid exploring already counted areas
