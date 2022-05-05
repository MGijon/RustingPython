"""En python tenemos diferentes tipos de datos:
   - Text Type:	 str
   - Numeric Types:	 int, float, complex
   - Sequence Types:	list, tuple, range
   - Mapping Type:	dict
   - Set Types:	set, frozenset
   - Boolean Type:	bool
   - Binary Types:	bytes, bytearray, memoryview
   - None Type:	 NoneType

REF: https://docs.python.org/3/library/stdtypes.html
"""
## OBS: podemos usar la funciOn 'type' para saber el tipo al que pertenece una variable!

## TIPOS SIMPLES
## =============
# str ------------------------------------------------------------------------
texto = "Hola Mundo"
print(type(texto))  # <class 'str'>

# Podemos acceder a los distintos caracteres que conforman el string
print(type(texto[0]))  # <class 'str'>

# int, float y complex -------------------------------------------------------
entero = 1
print(type(entero))  # <class 'int'>

decimal =  1.1
print(type(decimal))  # <class 'float'>

complejo = 10j
print(type(complejo))  # <class 'complex'>

# Booleano -------------------------------------------------------------------
booleano = True
print(type(booleano))  # <class 'bool'>

# bytes, bytearray, memoryview -----------------------------------------------
byte = b"Hello"
print(byte)  # b'Hello'
print(type(byte))  # <class 'bytes'>

byte_array = bytearray(4)
print(byte_array)  # bytearray(b'\x00\x00\x00\x00')
print(type(byte_array))  # <class 'bytearray'>

memoria = memoryview(bytes(5))
print(memoria)  # <memory at 0x7fa0d5277940>
print(type(memoria))  # <class 'memoryview'>

# None -----------------------------------------------------------------------
nada = None
print(type(nada))  # <class 'NoneType'>

## COMBINACIONES
## =============
# List -----------------------------------------------------------------------
lista = ["hola", " ", "mundo"]
print(lista)  # ['hola', ' ', 'mundo']
print(type(lista))  # <class 'list'>

# Algunos operadores estAn implementados, por ejemplo usar '+' para concatenar listas
lista = lista + ["!"]
print(lista)  # ['hola', ' ', 'mundo', '!']
print(type(lista))  # <class 'list'>

# Podemos combianar datos de diferentes tipos
#lista += texto  # fallarA, el operador '+' funciona entre elementos de la clase lista, contenga lo que contengan
lista += [entero, booleano]
print(lista)  # ['hola', ' ', 'mundo', '!', 1, True]
print(type(lista))  # <class 'list'>

# Podemos acceder a los diferentes elementos utilizando su Indice
print(lista[0])  # hola
print(type(lista[0]))  # <class 'str'>

print(lista[5])  # True
print(type(lista[5]))  # <class 'bool'>

# Tuple ----------------------------------------------------------------------
tupla = ("Hola", " ", "Mundo")
print(tupla)  # ('Hola', ' ', 'Mundo')
print(type(tupla))  # <class 'tuple'>

# las tuplas a differencia de las listas, son inmutables, una vez cradas no podremos modificarlas
print(tupla[0])  # Hola
#tupla[0] = 10  # TypeError: 'tuple' object does not support item assignment

# Range ----------------------------------------------------------------------
# DocumentaciOn: 
# The advantage of the range type over a regular list or tuple is that a range object will always take the same
# (small) amount of memory, no matter the size of the range it represents (as it only stores the start, stop and
# step values, calculating individual items and subranges as needed).
rango = range(10)
print(rango)  # range(0, 10)
print(type(rango))  # <class 'range'>
# Podemos acceder a los elementos convirtiEndolo en lista
print(list(rango))  # [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

# Set y frozenset ------------------------------------------------------------
conjunto = {"apple", "banana", "cherry"}
print(conjunto)  # {'apple', 'banana', 'cherry'}
print(type(conjunto))  # <class 'set'>
#print(conjunto[0])  # TypeError: 'set' object is not subscriptable

# La diferencia es que los "frozenset" son inmutables
conjunto_frozen = frozenset({"apple", "banana", "cherry"})
print(conjunto_frozen)  # frozenset({'apple', 'banana', 'cherry'})
print(type(conjunto_frozen))  # <class 'frozenset'>

# Dictionary -----------------------------------------------------------------
diccionario = {
    "key_1": "value 1",
    2: 3,
}
print(diccionario)  # {'key_1': 'value 1', 2: 3}
print(type(diccionario))  # <class 'dict'>

