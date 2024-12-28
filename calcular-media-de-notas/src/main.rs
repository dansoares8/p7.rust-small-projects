fn calcular_media(notas: &[f32]) -> f32{
    let tamanho = notas.len();

    let mut soma = 0.0;

    // para cada nota dentro do vetor de notas, vai somar todos os valores dentro de soma.
    //o asterisco é o operador de derreferência, para "nota" acessar o valor dentro do ponteiro "notas".

    for nota in notas{
        soma += *nota;
    }

    let media = soma / tamanho as f32;

    media
}

fn main(){
    // vetor de ponto flutuante
    let notas = [7.5, 8.0, 9.0, 6.5];

    // o &notas passara o valor para media por referência e não precisará criar uma cópia dos dados, economiza tempo e memória.
    let media = calcular_media(&notas);

    println!("A media das notas eh: {}", media);
}