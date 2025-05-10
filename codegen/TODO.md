# Todo

- [x] Rudimentary version (incorrect)
- [ ] Internal code for parameters in nested macro definitions.
- [ ] Macro table maybe should be a Set instead of a Map.

## 1

1. Passar por todo programa, tirar as definições de macro de primeiro nível e guardar na tabela de macros
2. Quando chamar um macro, pega na tabela e expande, depois faz o passo 1. novamente.

## 2

1. Passar por todo programa, tirar as definições de macro de primeiro nível e guardar na tabela de macros
2. Quando chamar um macro, pega na tabela e expande, depois tira somente as definições existirem no macro que vc acabou de expandir, já que as outras ja teriam sido tiradas antes.

## 3

1. Passar pelo programa e ir tirando definições de macro ao mesmo tempo que encontra chamadas de macro.
2. Ao encontrar chamadas de macro expande o macro se existir na tabela de macros e continua tirando as definições de macro.