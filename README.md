<img width="792" height="828" alt="20260910-0545-22 5960131" src="https://github.com/user-attachments/assets/2b94905d-d76e-4e23-8ffd-9e217d0f9056" />

# Conway's Game of Life

Implementación de Conway's Game of Life en Rust utilizando Raylib y un framebuffer propio.

El objetivo de este laboratorio es aplicar los conceptos de renderizado en tiempo real vistos en clase. Cada célula corresponde a un píxel del framebuffer y la simulación se actualiza generación por generación.

## Implementación

La simulación utiliza un framebuffer de 100x100 píxeles que se escala a una ventana de 800x800.

En cada generación:

1. Se revisan los 8 vecinos de cada célula.
2. Se aplican las reglas de Conway.
3. Se guarda el resultado en una nueva generación.
4. Se actualiza el framebuffer utilizando la función `point`.
5. Se muestra el framebuffer en la ventana usando `swap_buffers`.

El framebuffer no se limpia en cada frame, ya que la lógica del juego actualiza todas las células.

También se implementó wrapping en las orillas, por lo que los vecinos que salen de un extremo se toman del lado contrario.

## Patrones implementados

- Block
- Beehive
- Loaf
- Boat
- Blinker
- Toad
- Beacon
- Glider
- Lightweight Spaceship
- Pulsar
- Gosper Glider Gun

## Ejecutar

Para ejecutar el proyecto:

    cargo run

## Demo

![Game of Life Demo](demo.gif)
