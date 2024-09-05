fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let data_tripled: Vec<_> = data
    .iter()
    .map(|num| num * 3)
    .filter(|num| num > &10)
    .collect();

    for sample in data_tripled {
        println!("DATA SAMPLE: {:?}", sample);
    }
}