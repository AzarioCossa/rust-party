use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    // 1. Configuração e Leitura da Entrada
    let mut input = String::new();
    
    // Leitura da entrada padrão. Se falhar, termina o programa.
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Erro ao ler a entrada padrão.");
        return;
    }

    // Separa a entrada em dois blocos (infrações e cidadãos).
    // Assume que a quebra de linha dupla é o separador.
    let (infraction_block, citizens_block) = input.trim().split_once("\r\n\r\n").unwrap_or((input.trim(), ""));

    // 2. Inicialização das Estruturas de Dados (apenas tipos da std)
    let mut infraction_dictionary: HashMap<String, u8> = HashMap::new();
    // Chave: Cidadão, Valor: Lista de Infrações (para múltiplos registros)
    let mut citizens_data: HashMap<String, Vec<String>> = HashMap::new();
    // Chave: Cidadão, Valor: Pontuação de Crédito
    let mut result: HashMap<String, u8> = HashMap::new();

    // 3. Popula Dicionário de Infrações
    for line in infraction_block.lines() {
        let mut parts = line.split(":");
        if let (Some(infraction), Some(weight)) = (parts.next(), parts.next()) {
            let key = infraction.trim().to_string();
            
            // Converte a string de peso para u8, usando 0 em caso de falha de parsing.
            let value = weight.trim().parse::<u8>().unwrap_or(0);
            
            infraction_dictionary.insert(key, value);
        }
    }

    // 4. Popula Dados dos Cidadãos
    for line in citizens_block.lines() {
        let mut parts = line.split(":");
        if let (Some(citizen), Some(infraction)) = (parts.next(), parts.next()) {
            let citizen_key = citizen.trim().to_string();
            let infraction_value = infraction.trim().to_string();

            // Usa entry().or_insert_with(Vec::new) (função base) para:
            // 1. Obter uma referência mutável ao Vec (se a chave existir).
            // 2. Inserir um novo Vec vazio (se a chave não existir).
            // Em seguida, adiciona a infração ao Vec.
            citizens_data
                .entry(citizen_key)
                .or_insert_with(Vec::new)
                .push(infraction_value);
        }
    }

    // 5. Calcula a Pontuação Final
    // A pontuação inicial de 100 é definida aqui, usando iter_mut() para eficiência.
    for (citizen, infraction_list) in citizens_data.iter() {
        // Usa entry().or_insert(100) para iniciar a pontuação do cidadão em 100
        // e obter uma referência mutável ao valor.
        let credit_score = result.entry(citizen.clone()).or_insert(100);

        // Itera sobre cada infração do cidadão
        for infraction_name in infraction_list {
            // Busca o peso da infração. Usa .copied() e .unwrap_or(0) para obter um u8.
            let infraction_weight = infraction_dictionary
                .get(infraction_name)
                .copied()
                .unwrap_or(0); 

            // Atualiza a pontuação (referência mutável).
            // Usa saturating_sub (função base) para evitar underflow (pontuação < 0).
            *credit_score = credit_score.saturating_sub(infraction_weight);
        }
    }

    for (citizen, score) in result.clone(){
        if score<=0 {
            println!("{}:Stage gratuit", citizen);
        }else{
            println!("{}:{}", citizen, score);
        }
    }
}