use mysql::*;
use mysql::prelude::*;
use rand::Rng;
use dotenv::dotenv;
use std::env;

struct Collatz {
    dato: u64,
    pa: String,
}

impl Collatz {
    fn secuencia(&self, da: u64) -> Vec<u64> {
        let mut a: Vec<u64> = Vec::new();
        let mut wi = da;
        while wi != 1 && wi != 2 && wi != 4 {
            if wi % 2 == 0 {
                wi /= 2;
            } else {
                wi = (wi * 3) + 1;
            }
            a.push(wi);
        }
        a
    }

    fn encriptar(&self, mensaje: &str, secuencia: &Vec<u64>) -> String {
        let alfabeto: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_+={}[]\\|;:'\",.<>?/ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõöøùúûüýþÿ".chars().collect();
        let mut resultado = String::new();
        
        for (i, ch) in mensaje.chars().enumerate() {
            if i < secuencia.len() {
                let des = secuencia[i] as usize;
                if let Some(pos) = alfabeto.iter().position(|&c| c == ch) {
                    let nueva_pos = (pos + des) % alfabeto.len();
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
    dotenv().ok();

    let user = env::var("MYSQL_USER").expect("MYSQL_USER no definido");
    let password = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD no definido");
    let host = env::var("MYSQL_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string());
    let database = env::var("MYSQL_DATABASE").expect("MYSQL_DATABASE no definido");

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(port.parse::<u16>().unwrap_or(3306))
        .user(Some(user))
        .pass(Some(password))
        .db_name(Some(database));

    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;

    let mut rng = rand::thread_rng();
    let mut semilla: u64 = rng.gen_range(80..400);
    if semilla > 130 {
        semilla = rng.gen_range(140..=600);
    }

    let password_original = "perfume123";
    let email = "correo@ejemplo.com";
    let nombre = "Juan Perez";
    let usuario = "juan123";

    let collatz = Collatz {
        dato: semilla,
        pa: password_original.to_string(),
    };

    let secuencia = collatz.secuencia(semilla);
    let password_encriptado = collatz.encriptar(password_original, &secuencia);

    conn.exec_drop(
        r"INSERT INTO user_table
         (create_time_user, update_time_user, password_user, email_user, name_user, user_user, collatz_seed)
         VALUES (NOW(), NOW(), :password, :email, :nombre, :usuario, :collatz_seed)",
        params! {
            "password" => password_encriptado,
            "email" => email,
            "nombre" => nombre,
            "usuario" => usuario,
            "collatz_seed" => semilla,
        },
    )?;

    println!("Usuario insertado correctamente con contraseña encriptada.");
    Ok(())
}