struct Analisador{
    entrada: String,
    posicao: usize
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
fn remove(s: &mut String, i: usize) -> String{
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
        }
    }
    
    fn próximo_car(&mut self) -> (usize, String){
        let c = remove(&mut self.entrada,0);
        let p = self.posicao;
        self.posicao +=1;
        (p, c)
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
                                        return (true, self.posicao - 1, c.to_string());
                                     } ,
                        '0' ..='9' => {
                            numero.push(c);
                            let mut contador = 1;
                            loop {
                              //  println!("{}",self.entrada);
                               // println!("cc {}",self.posicao + contador);
                                match self.entrada.chars().nth((self.posicao + contador)-1){
                                    None => return (true, self.posicao - 1, numero),
                                    Some(c) => {
                                       // println!("AA: {c}");
                                        if c.is_numeric() {
                                            numero.push(c);
                                            contador += 1;
                                        } else {
                                            return (true, self.posicao - 1, numero);
                                        }
                                    }
                                }
                            }
                        }
                        ' ' => continue,    
                        _ => return (false, self.posicao - 1, c.to_string()),
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


fn main() {
    let entrada = String::from("240 + 20");  
    let mut a = Analisador::novo(entrada);
    loop{
        let (b,u,s) = a.próximo();
        println!("('{s}',{u})");
        if !b{
            break;
        }
        //println!("{}",a.entrada.clone());
        if is_empty(a.entrada.clone()){
            break;
        }
        
        let (mut p, mut c): (usize, String) = (0,"0".to_string());
        for _i in 0..s.len(){
            println!("{}",s.len());
            (p,c) = a.próximo_car();
        }
        println!("{p} ee {c}");
        a.devolver(p,c); // Devolve para o analisador
        //Passa para o próximo
    }
    println!("Finalizado!");
}