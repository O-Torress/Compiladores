int main(void) {
    double x = 12;
    double num1, num2;
    char operador[3];
    char continuar = '1';

    while (continuar == '1' ) {
        printf("Ingrese el primer numero: ");
        if (scanf("%lf", &num1) != 1) {
            printf("Entrada invalida. Reinicie el programa.\n");
            return 1;
        }

        printf("Ingrese el segundo numero: ");
        if (scanf("%lf", &num2) != 1) {
            printf("Entrada invalida. Reinicie el programa.\n");
            return 1;
        }

        printf("Ingrese el operador (+, -, /, *, %%): ");
        scanf(" %2s", operador);

        if (operador[0] == '+' && operador[1] == '\0') {
            printf("El resultado de la suma es: %.6g\n", num1 + num2);
        } else if (operador[0] == '-' && operador[1] == '\0') {
            printf("El resultado de la resta es: %.6g\n", num1 - num2);
        } else if (operador[0] == '*' && operador[1] == '\0') {
            printf("El resultado de la multiplicacion es: %.6g\n", num1 * num2);
        } else if (operador[0] == '/' && operador[1] == '\0') {
            if (num2 == 0.0) {
                printf("Error: division por cero.\n");
            } else {
                printf("El resultado de la division es: %.6g\n", num1 / num2);
            }
        } else if (operador[0] == '%' && operador[1] == '\0') {
            if (num2 == 0.0) {
                printf("Error: residuo con divisor cero.\n");
            } else {
                printf("El residuo de la division es: %.6g\n", fmod(num1, num2));
            }
        } else {
            printf("Operador incorrecto. Usa +, -, /, *, %%.\n");
        }

        printf("Quieres continuar ? (0/1): ");
        scanf(" %c", &continuar);
        printf("\n");
    }

    printf("Gracias por usar la calculadora.\n");
    return 0;
}