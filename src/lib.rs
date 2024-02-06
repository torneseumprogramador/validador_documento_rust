pub fn validar_cpf(cpf: &str) -> bool {
    let cpf = cpf.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
    if cpf.len() != 11 || cpf.iter().all(|&d| d == cpf[0]) {
        return false;
    }
    let mut soma = 0;
    for (i, &digit) in cpf.iter().take(9).enumerate() {
        soma += digit.to_digit(10).unwrap() * (10 - i as u32);
    }
    let resto = (soma * 10) % 11 % 10;
    if resto != cpf[9].to_digit(10).unwrap() {
        return false;
    }
    soma = 0;
    for (i, &digit) in cpf.iter().take(10).enumerate() {
        soma += digit.to_digit(10).unwrap() * (11 - i as u32);
    }
    let resto = (soma * 10) % 11 % 10;
    resto == cpf[10].to_digit(10).unwrap()
}


pub fn validar_cnpj(cnpj: &str) -> bool {
    let cnpj = cnpj.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
    if cnpj.len() != 14 || cnpj.iter().all(|&d| d == cnpj[0]) {
        return false;
    }

    let pesos = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let soma: u32 = cnpj.iter().take(12).enumerate().map(|(i, &digit)| digit.to_digit(10).unwrap() * pesos[i + 1]).sum();
    let resto = (soma % 11) as u8;
    let dv1 = if resto < 2 { 0 } else { 11 - resto };

    if dv1 != cnpj[12].to_digit(10).unwrap() as u8 {
        return false;
    }

    let soma: u32 = cnpj.iter().take(13).enumerate().map(|(i, &digit)| digit.to_digit(10).unwrap() * pesos[i]).sum();
    let resto = (soma % 11) as u8;
    let dv2 = if resto < 2 { 0 } else { 11 - resto };

    dv2 == cnpj[13].to_digit(10).unwrap() as u8
}

