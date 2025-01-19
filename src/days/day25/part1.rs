const ROW: usize = 3010;
const COL: usize = 3019;
pub fn run() -> usize {
    let mut row = 1;
    let mut num: usize = 20151125;
    loop {
        let mut ii = row;
        let mut jj = 1;

        while jj <= row {
            if ii == ROW && jj == COL {
                return num;
            }
            num = (num * 252533).rem_euclid(33554393);
            jj += 1;
            ii -= 1;
        }
        row += 1;
    }
}
