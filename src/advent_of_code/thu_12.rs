use super::*;

pub fn garden_groups(input: &str) -> usize {
    let garden = utils::read_garden_arrangement_input(input);

    let plot_list = get_plot_list(&garden);

    let mut total_cost: usize = 0;

    for plot in plot_list {
        println!("Plot: {:?}", plot);
        total_cost += plot.side_count * plot.plant_list.len();
    }

    total_cost
}

pub fn get_plot_list(map: &Garden) -> PlotList {
    let mut plot_list: PlotList = vec![];

    for (row_index, row) in map.iter().enumerate() {
        for (plant_index, plant) in row.iter().enumerate() {
            let new_coord = Coordinate {
                x: plant_index,
                y: row_index,
            };
            // FIXME: this does not cover all the cases! Any linear plot wont be covered
            if row_index > 0 {
                if let Some(plot_index) = which_plot_contains_coords(
                    &plot_list,
                    Coordinate {
                        x: plant_index,
                        y: row_index - 1,
                    },
                ) {
                    if plot_list[plot_index].plant == *plant {
                        plot_list[plot_index].plant_list.push(new_coord);
                        plot_list[plot_index].side_count += calc_sides_of_plant(
                            map,
                            &Coordinate {
                                x: plant_index,
                                y: row_index,
                            },
                            plant,
                        );
                        continue;
                    }
                }
            }
            if plant_index > 0 {
                if let Some(plot_index) = which_plot_contains_coords(
                    &plot_list,
                    Coordinate {
                        x: plant_index - 1,
                        y: row_index,
                    },
                ) {
                    if plot_list[plot_index].plant == *plant {
                        plot_list[plot_index].plant_list.push(new_coord);
                        plot_list[plot_index].side_count += calc_sides_of_plant(
                            map,
                            &Coordinate {
                                x: plant_index,
                                y: row_index,
                            },
                            plant,
                        );

                        continue;
                    }
                }
            }

            plot_list.push(Plot {
                plant: *plant,
                side_count: calc_sides_of_plant(map, &new_coord, plant),
                plant_list: vec![new_coord],
            });
        }
    }

    plot_list
}

pub fn calc_sides_of_plant(map: &Garden, coord: &Coordinate, plant: &Plant) -> usize {
    let mut sum: usize = 0;

    if coord.x == 0 || map[coord.y][coord.x - 1] != *plant {
        sum += 1;
    }

    if coord.y == 0 || map[coord.y - 1][coord.x] != *plant {
        sum += 1;
    }

    if coord.x < map[0].len() - 2 && map[coord.y][coord.x + 1] != *plant
        || coord.x == map[0].len() - 1
    {
        sum += 1;
    }

    if coord.y < map.len() - 2 && map[coord.y + 1][coord.x] != *plant || coord.y == map.len() - 1 {
        sum += 1;
    }

    sum
}

pub fn which_plot_contains_coords(list: &PlotList, coord: Coordinate) -> Option<usize> {
    for (plot_index, plot) in list.iter().enumerate() {
        if plot.plant_list.contains(&coord) {
            return Some(plot_index);
        }
    }

    None
}

// Perimeter is calculated looping over the elements and adding every non-member.
//
// mutate the garden on every iteration to avoid exploring already counted areas
