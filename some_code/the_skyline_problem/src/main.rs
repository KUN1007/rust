struct Building {
    left: i32,
    right: i32,
    height: i32,
}

fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut points: Vec<(i32, i32)> = Vec::new();
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut buildings: Vec<Building> = buildings
        .into_iter()
        .map(|b| Building {
            left: b[0],
            right: b[1],
            height: b[2],
        })
        .collect();

    buildings.sort_by(|a, b| a.left.cmp(&b.left));

    let mut heights: Vec<i32> = Vec::new();
    heights.push(0);

    for building in buildings {
        let left = building.left;
        let right = building.right;
        let height = building.height;

        while let Some(last) = heights.last() {
            if *last <= left {
                heights.pop();
            } else {
                break;
            }
        }

        if let Some(last) = heights.last_mut() {
            if *last < height {
                points.push((left, height));
                *last = height;
            }
        }

        if let Some((_, prev_height)) = points.last() {
            if *prev_height != height {
                points.push((right, *prev_height));
                points.push((right, height));
            }
        } else {
            points.push((left, height));
            points.push((right, height));
        }
    }

    for (x, y) in points {
        if result.last().map(|p| p[1]) != Some(y) {
            result.push(vec![x, y]);
        }
    }

    result
}

fn main() {
    let buildings = vec![
        vec![2, 9, 10],
        vec![3, 7, 15],
        vec![5, 12, 12],
        vec![15, 20, 10],
        vec![19, 24, 8],
    ];

    let skyline = get_skyline(buildings);

    for point in skyline {
        println!("{:?}", point);
    }
}
