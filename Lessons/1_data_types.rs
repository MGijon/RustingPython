#[allow(dead_code)]  // No mostrar warnings por cOdigo sin usar
#[allow(unused_variables)]  // No mostrar codigo por variables no en uso

// OBS: usaremos la macro (luego explicaremos esto) 'println!()' para imprimir por pantalla
    // Hay distintos tipos de macros para 'escupir' al standard output, ya lo veremos.
// OBS: Para declarar una variable 'let nombreVariable: tipo_de_dato = valor;'
// OBS: acabamos cada lInea con ';'
// REF: https://doc.rust-lang.org/book/ch03-02-data-types.html

fn main() {
    // SCALAR TYPES: integer, floating-point numbers, booleans and characters
    // ------------
    // (1) INTEGERS -> signed (i) y unsigned (u) de 8 a 64 bits, isize/usize
    println!("Hello, world!");
    let a: u8 = 1;  // integer unsigned de 8 bits
    println!("{}", a);
    //a = 2;  // fallarA, la variables en Rust no son mutables por defecto

    // isize, usize
    // TODO

    // OBS: por defecto el tipo es i32, podemos no indicarlo explIcitamente
    let aa = 2;
    println!("{}", aa);

    // podemos inicializar la variable usando distintas bases
    let a_decimal: u32 = 98_222;
    let a_hex: u32 = 0xff;
    let a_octal: u32 = 0o77;
    let a_binary: u32 = 0b1111_0000;
    let a_byte: u8 = b'A'; // solo para u8

    println!(
        "{} {} {} {} {}",
        a_decimal, a_hex, a_octal, a_binary, a_byte
    );
    // (2) FLATING-POINT NUMBERS -> f32 y f64
    let x = 3.3;  // por defecto f64
    let y: f32 = 1.1;

    println!("{} {}", x, y);
    println!(
        "suma: {} - resta: {} - división: {} - multiplicación: {} - resto: {}",
        x+y, x-y, x/y, x*y, x%y
    );

    // (3) BOOLEANS
    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    // (4) CHARACTERS
    let z = 'z';  // CUIDADO! "" y '' no son intercambiables, "" genera un &str
    let zz: char = 'ℤ';
    let hearts: char = '😻';
    println!("{} {} {}", z, zz, hearts);

    // COMPOUND TYPES: tuples and arrays
    // ---------------
    // (1) Tuples
    // - Permite agrupar variables de distintos tipos
    // - Is inmutable
    // - Podemos acceder a sus diferentes elementos utilizando '.'
    // - Las declaramos usando parEntesis.
    let tupla1: (u32, f64, u8);
    tupla1 = (73, 9.0, 8);
    println!("{:?}", tupla1);  // no podemos usar el formateador por defecto
    println!("{} - {} - {}", tupla1.0, tupla1.1, tupla1.2);

    // podemos desempaquetar las variables
    let (x, y, z) = tupla1;
    println!("{} {} {}", x, y, z);

    // (2) ARRAY
    // - Agrupa elementos del mismo tipo
    // - La longitud es fija (a diferencia de Python!)
    let ar1: [i32; 3] = [3, 2, 1];
    let ar2 = [1, 2, 3, 4];  // el tipo puede inferirse

    // podemos crear arrays con un Unico valor repetido x veces, 
    let ar3 = [1; 10];  // 1, 1, 1, 1, ..., 1 - 10 veces
    
    // accedemos a los elementos como lo harIamos en Python, con corchetes
    println!("{} {}", ar1[0], ar2[3]);
}
// https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
