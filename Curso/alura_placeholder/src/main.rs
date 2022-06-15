const PI:f32 = 3.14;
static mut GLOBAL:u8 = 1;

fn soma(a: i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }

    println!("fora, a = {}", a);
}

fn escopo() {
    println!("PI = {}", PI);
    unsafe {
        println!("variavel_global = {}", GLOBAL);
    }

    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));
    let variavel:i32 = 301;
    println!("variavel = {}, tamanho = {}", variavel, std::mem::size_of_val(&variavel));

    let decimal:f32 = 2.5;
    println!("decimal = {}, tamanho = {}", decimal, std::mem::size_of_val(&decimal));


    let booleana:bool = true;
    println!("booleana = {}, tamanho = {}", booleana, std::mem::size_of_val(&booleana));
    
    let letra:char = 'C';
    println!("letra = {}, tamanho = {}", letra, std::mem::size_of_val(&letra));

}

fn main() {
    escopo();
    sombra();
    println!("Soma = {}", soma(1, 2));

    condicionais();
    repeticoes();

    ownership();

    pattern_matching();
    erros();
}

fn condicionais() {
    let idade: u8 = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou{
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao = if eh_maior { "maior"} else {"menor"};

    println!("é {} de idade", condicao);

    let linguagem = "C#";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O proposito de {} eh {}", linguagem, proposito);
}

fn repeticoes() {
    let multiplicador: u8 = 5;

    let mut contador:u8 = 0;
    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;
        }

        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10 {
            break;
        }
    } 

    for i in 1..=10 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn ownership() {
    let mut uma_string = String::from("Vinicius");
    rouba(&mut uma_string);
    
    println!("{}", uma_string);
}

fn rouba(string: &mut String) {
    string.push_str(" Dias");
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um Pouquinho",
            4..=10 => "Um Bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        })
    }
}

fn erros() {
    match resultado() {
        Ok(s) => println!("String de Sucesso = {}", s),
        Err(numero) => println!("Código de erro = {}", numero)
    }
}

fn resultado() -> Result<String, u8> {
    Err(42)
}