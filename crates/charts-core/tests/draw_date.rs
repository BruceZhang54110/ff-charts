#[cfg(test)]
mod tests {
    use chrono::{Duration, Local};
    use rand::prelude::*;

    #[test]
    fn print_date() {
        let now = Local::now();
        let range = 1..8;
        range.into_iter().for_each(|i| {
            let date = now + Duration::days(i - 7);
            println!("i-7 is: {:?}, date is: {:?}", i - 7, date.format("%Y-%m-%d").to_string());
        });

        let dates: Vec<String> = (1..8).map(|i| -> String { 
                let date = now + Duration::days(i - 7);
                date.format("%Y-%m-%d").to_string()
            }).collect();
        println!("now_time: {:?}", now);
        
        println!("dates len is: {:?}, value is :{:?}", dates.len(), dates);

        let datas: Vec<i32> = dates.into_iter().map(|_| rand::rng().random_range(1..100)).collect();
        println!("a is: {:?}", datas);
    }

    
}