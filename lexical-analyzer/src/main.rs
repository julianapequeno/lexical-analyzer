struct Analisador{
    entrada: String,
    posicao: usize,
    tam: usize
}

/// Retorna se a string é vazia.
fn is_empty(s: String) -> bool{
    if s.len() == 0{
        return true;
    }
    return false;
}
/// Remove o caracter na posição (em bytes) `i`
/// e o retorna.
fn remove(s: &mut String) -> String{
    let mut result = String::new();
    *s = s[1..].to_string();

    for (index, c) in s.chars().enumerate() {
        if index == 0 {
            continue;
        } else {
            result.push(c);
        }

    }

    return result;
}

impl Analisador {
    fn novo(entrada: String) -> Self {
        Analisador {
            entrada,
            posicao: 0,
            tam: 0,
        }
    }
    
    fn próximo_car(&mut self) -> String{
        let c = remove(&mut self.entrada);
        //let p = self.posicao;
        self.posicao +=1;
        c
    }
        
    fn próximo(&mut self) -> (bool, usize, String){
        let mut numero = String::new();
        loop {
            match self.entrada.chars().nth(self.posicao) {
                None => return (true, self.posicao, String::new()),
                Some(c) => {
                    self.posicao += 1;
                    match c {
                        '+' | '-' => {  self.posicao += 1;
                                        self.tam += numero.len();
                                        return (true, (self.posicao - 1), c.to_string());
                                     } ,
                        '0' ..='9' => {
                            numero.push(c);
                            let mut contador = 1;
                            loop {
                                match self.entrada.chars().nth((self.posicao + contador)-1){
                                    None => return (true, (self.posicao - 1), numero),
                                    Some(c) => {
                                        if c.is_numeric() {
                                            numero.push(c);
                                            contador += 1;
                                        } else {
                                            self.tam += numero.len();
                                            return (true, (self.posicao - 1), numero);
                                        }
                                    }
                                }
                            }
                        }
                        ' ' => continue,    
                        _ => {
                            self.tam += 1;
                            return (false, (self.posicao - 1), c.to_string());
                        }
                    }
                },
            }
        }
    }

    fn devolver(&mut self, pos: usize, s: String) {
        //caso precise analisar um elemento léxico à frente.
        self.entrada = s;
        self.posicao = pos;
    }
}

fn leitura() -> String{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    return s;
}


fn engine(entrada: String){
    let mut a = Analisador::novo(entrada);
    loop{
        let (b,u,s) = a.próximo();
        print!("('{s}',{u}) ");
        if !b{
            println!("Erro na posição {u}");
            break;
        }
        if is_empty(a.entrada.clone()){
            break;
        }
        
        let mut c: String = "0".to_string();
        for _i in 0..s.len(){
            c = a.próximo_car();
        }
        a.devolver(0,c); // Devolve para o analisador
        //Passa para o próximo
    }
    println!("");
}


fn main() {
    loop{
        println!("> Press Ctrl+C if you want to exit the program");

        let entrada = String::from(leitura());  

        engine(entrada);
    }
}