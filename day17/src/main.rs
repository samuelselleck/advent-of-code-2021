fn main() {
    //(209..=238, -86..=-59);
    let (tsx, tex, tsy, tey): (i32, i32, i32, i32) = (209, 238, -86, -59);//
    let max_height = find_max_height(tsx, tex, tsy, tey);
    println!("{}", max_height)
}

fn find_max_height(tsx: i32, tex: i32, tsy: i32, tey: i32) -> i32 {
    let mut num = 0;
    for dx_s in 0..300 {
        for dy_s in -1000..2000 {
            let (mut x, mut y) = (0, 0);
            let (mut dx, mut dy): (i32, i32) = (dx_s, dy_s);
            while dy >= 0 || y > tsy {
                x += dx;
                y += dy;
                dx -= dx.signum();
                dy -= 1;
                if (tsx..=tex).contains(&x) && (tsy..=tey).contains(&y) {
                    num += 1;
                    break;
                }
            }
        }
    }
    num
}
