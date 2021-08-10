#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use rand::Rng;
use rand::seq::SliceRandom;


#[tauri::command]
fn _generator() -> String{
 
  let mut rng = rand::thread_rng();
  let mut _posicion_abc = 0;
  let mut _posicion_numeros = 0;
  let mut _posicion_caracteres = 0;
 
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

  let mut _password = String::new();

  let mut _final_result = String::new();



  let mut _i: u8 = 0;


  for _i in 0..3 {
      _posicion_abc = rng.gen_range(0..26);
      _posicion_numeros = rng.gen_range(0..9);

      let _abc_mayuscula_valor: &str = _abecedario[_posicion_abc];
      let _abc_minuscula_valor = _abecedario[_posicion_numeros].to_lowercase();
      let _abc_minuscula_valor= _abc_minuscula_valor.as_str();

      let _numero_valor: &str = _numeros[_posicion_numeros];

      _password.push_str(_abc_mayuscula_valor);
      _password.push_str(_abc_minuscula_valor);
      _password.push_str(_numero_valor);

      if _i == 0 {
          _posicion_caracteres = rng.gen_range(0..13);
          let _caracter_valor: &str = _caracteres[_posicion_caracteres];
          _password.push_str(_caracter_valor);

      }

      if _i == 2 {
          _posicion_caracteres = rng.gen_range(0..13);
          let _caracter_valor: &str = _caracteres[_posicion_caracteres];
          _password.push_str(_caracter_valor);

      }

      let mut bytes = _password.to_string().into_bytes();
      bytes.shuffle(&mut rng);
      _final_result = String::from_utf8(bytes).unwrap();
     

  }

  _final_result


}

fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![_generator])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
