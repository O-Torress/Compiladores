#include <iostream>
#include <string>
using namespace std;

int main (){
  
  float num1;
  float num2;
  string Operador;

  cout <<"Ingrese el primer numero: ";
  cin >> num1;

  cout <<"\n Ingrese el segundo numero: ";
  cin >> num2;

  cout <<"\n Ingrese el Operador (+, -, /, *): ";
  cin >> Operador;

  if (Operador == "+") {
    cout << "\n El resultado de la suma es: " << (num1 + num2) << endl ;
  }else if (Operador == "-"){
    cout << "\n El resultado de la resta es: "<< (num1 - num2) << endl ;
  }else if (Operador == "/"){
    cout << "\n El resultado de la division es: "<< (num1 / num2) << endl ;
  }else if (Operador == "*"){
    cout << "\n El resultado de la multiplicacion es: "<< (num1 * num2) << endl ;
  }else{
    cout << "\n Operador incorrecto";
  }

} 