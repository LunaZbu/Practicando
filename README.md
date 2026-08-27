# Practicando 🦀

Proyecto de práctica en **Rust** enfocado en entender cómo funciona una conexión remota vía HTTP.

## ¿Qué hace?

Permite controlar el estado de la pantalla de un PC (encender/apagar) de forma remota desde el celular, a través de una conexión HTTP en red local.

## Tecnologías

- **Rust**
- HTTP (conexión remota local)
- Git / Git Bash

## Motivación

Este proyecto nació como ejercicio práctico para entender el flujo de una conexión remota HTTP: cómo se levanta un servidor simple, cómo se reciben peticiones y cómo esas peticiones ejecutan una acción real en el sistema.

## Estado

✅ Finalizado — funcional para uso local/personal.

## Cómo correrlo

```bash
git clone https://github.com/LunaZbu/Practicando.git
cd Practicando
cargo run
```

> Requiere tener [Rust y Cargo](https://www.rust-lang.org/tools/install) instalados.

## Próximos pasos (ideas)

- [ ] Agregar autenticación básica a las peticiones
- [ ] Soporte para más acciones remotas (no solo pantalla)
