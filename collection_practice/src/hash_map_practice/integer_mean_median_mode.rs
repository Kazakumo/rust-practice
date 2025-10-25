use std::collections::HashMap;

pub fn integer_mean_median_mode(vec: &mut Vec<i32>) {
    // から配列の時はなんか入れろと命令してreturn
    if vec.is_empty() {
        println!("insert into vec!");
        return;
    }
    // なんか入っているときは平均中央最頻を求める
    // 平均を求める
    let mut tmp = 0;
    for v in vec.iter() {
        // NOTE: iter()を呼ぶことで、
        tmp += v;
    }

    let mean = tmp / vec.len() as i32;
    println!("mean is {}", mean);

    // 中央値を求める
    if vec.len() % 2 == 0 {
        // 要素数が偶数の時
        vec.sort();
        let median = (vec[vec.len() / 2] + vec[vec.len() / 2 - 1]) / 2; // NOTE: -1なのは、lenが要素数 + 1の値を示すから
        println!("median is {}", median);
    } else {
        // 要素数が奇数の時
        vec.sort();
        let median = vec[vec.len() / 2];
        println!("median is {}", median);
    }

    // 最頻値を求める
    let mut appeared_numbers: HashMap<i32, i32> = HashMap::new();
    // まずは数字と出現回数のMapを作るイテレーションする
    for v in vec.iter() {
        let count = appeared_numbers.entry(*v).or_insert(0);
        *count += 1;
    }
    let max_value = appeared_numbers.values().max();
    match max_value {
        Some(v) => {
            for (key, value) in appeared_numbers.iter() {
                if value == v {
                    println!("mode is {}", key);
                }
            }
        }
        None => {
            println!("map is empty")
        }
    }
}
