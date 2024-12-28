fn main(){

    println!("Por favor, digite um número inteiro:");

    let mut input = String::new();

    //ler o input do usuário:
    std::io::stdin().read_line(&mut input).expect("Falha ao ler o número.");

    let mut numbers: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map( |x| x.parse::<f64>().expect("Por favor, insira números reais.") )
        .collect();

    let mut sum: f64 = 0.0;

    for num in &numbers{
        if num % 2.0 == 0.0{
            sum += num;
        }
    }

    println!("A soma dos numeros pares eh: {}", sum);

}