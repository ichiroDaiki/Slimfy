#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use rand::Rng;
use rand::seq::SliceRandom;

fn shuffle(x: &mut String) -> String{
      let mut rng = rand::thread_rng();
      let mut bytes = x.clone().into_bytes();
      bytes.shuffle(&mut rng);
      let shuffle_password = String::from_utf8(bytes).unwrap();
      shuffle_password
}

#[tauri::command]
fn _generator() -> String{
 
  let mut rng = rand::thread_rng();
  let _abecedario = [
      "A", "B", "C", "D", "E", "F", "G",
      "H", "I", "J", "K", "L", "M", "N",
      "R", "O", "P", "Q", "R", "S", "T", "U",
      "V", "W", "X", "Y", "Z"
  ];

  let _numeros = [
      "1", "2", "3", "4", "5", "6", "7",
      "8", "9", "0"
  ];

  let _caracteres = ["",
  "-", "*", "?", "!", "@", "#", "",
  "$", "=", ".", ";", ":", ""];

  let mut generate_password : String = String::new();
  let mut _i: u8 = 0;

  for _i in 0..3 {

      let _abc_mayuscula_valor: &str = _abecedario[rng.gen_range(0..26)];
      let _abc_minuscula_valor: String = _abecedario[rng.gen_range(0..9)].to_lowercase();
      let _abc_minuscula_valor: &str = _abc_minuscula_valor.as_str();

      let _numero_valor: &str = _numeros[rng.gen_range(0..9)];

      generate_password.push_str(_abc_mayuscula_valor);
      generate_password.push_str(_abc_minuscula_valor);
      generate_password.push_str(_numero_valor);

  }
  
  let _caracter_valor: &str = _caracteres[rng.gen_range(0..13)];
  generate_password.push_str(_caracter_valor);
  let contra : String = shuffle(&mut generate_password);

  contra

}


fn main() {
  tauri::Builder::default()

    .invoke_handler(tauri::generate_handler![_generator])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
