pub fn minmax(data: &Vec<(i32, i32)>) -> (i32, i32, i32, i32) {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (data[0].0, data[0].0, data[0].1, data[1].1);

    for (x, y) in data {
        x_min = std::cmp::min(*x, x_min);
        x_max = std::cmp::max(*x, x_max);
        y_min = std::cmp::min(*y, y_min);
        y_max = std::cmp::max(*y, y_max);
    }

    (x_min, x_max, y_min, y_max)
}
