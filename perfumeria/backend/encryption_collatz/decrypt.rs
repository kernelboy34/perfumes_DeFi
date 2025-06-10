use mysql::*;
use mysql::prelude::*;

struct Collatz;

impl Collatz {
    fn secuencia(semilla: u64) -> Vec<u64> {
        let mut secuencia = Vec::new();
        let mut valor = semilla;

        while valor != 1 && valor != 2 && valor != 4 {
            if valor % 2 == 0 {
                valor /= 2;
            } else {
                valor = valor * 3 + 1;
            }
            secuencia.push(valor);
        }

        secuencia
    }

    fn desencriptar(encriptado: &str, secuencia: &[u64]) -> String {
        let alfabeto: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_+={}[]\\|;:'\",.<>?/ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõöøùúûüýþÿ"
            .chars()
            .collect();

        let mut resultado = String::new();

        for (i, ch) in encriptado.chars().enumerate() {
            if i < secuencia.len() {
                let des = secuencia[i] as usize;
                if let Some(pos) = alfabeto.iter().position(|&c| c == ch) {
                    let nueva_pos = (pos + alfabeto.len() - (des % alfabeto.len())) % alfabeto.len();
                    resultado.push(alfabeto[nueva_pos]);
                } else {
                    resultado.push(ch);
                }
            }
        }

        resultado
    }
}

fn main() -> Result<()> {
    let url = "mysql://perfumesadmin:perfume_admin@127.0.0.1:3308/perfume_datos";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let row: Option<(String, u64)> = conn.exec_first(
        "SELECT password_user, collatz_seed FROM user_table WHERE user_user = :usuario",
        params! {
            "usuario" => "juan123",
        },
    )?;

    if let Some((password_encriptado, semilla)) = row {
        let secuencia = Collatz::secuencia(semilla);
        let password_desencriptado = Collatz::desencriptar(&password_encriptado, &secuencia);
        println!("Contraseña desencriptada: {}", password_desencriptado);
    } else {
        println!("Usuario no encontrado.");
    }

    Ok(())
}
