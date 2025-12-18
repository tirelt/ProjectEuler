fn main() {
    let thirty = [4,6,9,11];
    let mut year: u32 = 1900;
    let mut month: u32 = 1;
    let mut current_start_month = 1;//sunday = 0 
    let mut res = 0; 
    let mut days = 0;
    while year <2001 {
        if month == 2{
            if year % 4 == 0 && !(year % 100 == 0 ){
                days = 29;
            } else {
               days = 28; 
            }
        } else {
            if thirty.contains(&month) {
                days = 30;
            } else {
                days = 31;
            }
        }
        current_start_month = (&days + &current_start_month)%7;
        if month == 12{
            month = 1;
            year += 1;
        } else {
            month += 1;
        }    
        if current_start_month == 0 && year>1900 && year < 2001 {
            res += 1
        }
    }
    println!("{res}");
}
