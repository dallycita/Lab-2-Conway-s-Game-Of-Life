mod framebuffer;
mod point;

use framebuffer::Framebuffer;
use point::point;

use raylib::prelude::*;

use std::thread;
use std::time::Duration;

// ---------------------------------------------------------
// COLORES
// ---------------------------------------------------------

fn alive_color() -> Color {
    Color::new(255, 230, 50, 255)
}

fn dead_color() -> Color {
    Color::new(45, 0, 65, 255)
}

// ---------------------------------------------------------
// GAME OF LIFE
// ---------------------------------------------------------

fn is_alive(framebuffer: &Framebuffer, x: i32, y: i32) -> bool {
    framebuffer.get_color(x, y) == alive_color()
}

// Conecta las orillas del framebuffer.
// Si salimos por un lado, entramos por el lado contrario.
fn wrap(value: i32, max: i32) -> i32 {
    if value < 0 {
        max - 1
    } else if value >= max {
        0
    } else {
        value
    }
}

// Cuenta cuántos de los 8 vecinos están vivos.
fn count_alive_neighbors(framebuffer: &Framebuffer, x: i32, y: i32) -> u8 {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            // No contamos la célula del centro.
            if dx == 0 && dy == 0 {
                continue;
            }

            let neighbor_x = wrap(x + dx, framebuffer.width());

            let neighbor_y = wrap(y + dy, framebuffer.height());

            if is_alive(framebuffer, neighbor_x, neighbor_y) {
                count += 1;
            }
        }
    }

    count
}

// Reglas de Conway:
//
// 1. Viva con menos de 2 vecinos -> muere.
// 2. Viva con 2 o 3 vecinos -> sobrevive.
// 3. Viva con más de 3 vecinos -> muere.
// 4. Muerta con exactamente 3 vecinos -> nace.
fn next_state(currently_alive: bool, neighbors: u8) -> bool {
    if currently_alive {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
}

// Calcula una generación completa.
fn render(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width();
    let height = framebuffer.height();

    // Guardamos la siguiente generación aparte.
    //
    // No podemos modificar las células mientras contamos
    // vecinos porque todas deben cambiar al mismo tiempo.
    let mut next_generation = vec![false; (width * height) as usize];

    // Primero calculamos toda la siguiente generación.
    for y in 0..height {
        for x in 0..width {
            let currently_alive = is_alive(framebuffer, x, y);

            let neighbors = count_alive_neighbors(framebuffer, x, y);

            let alive_next = next_state(currently_alive, neighbors);

            let index = (y * width + x) as usize;

            next_generation[index] = alive_next;
        }
    }

    // Después de terminar los cálculos,
    // actualizamos todos los píxeles.
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;

            if next_generation[index] {
                framebuffer.set_current_color(alive_color());
            } else {
                framebuffer.set_current_color(dead_color());
            }

            // Usamos únicamente point para pintar.
            point(framebuffer, Vector2::new(x as f32, y as f32));
        }
    }
}

// ---------------------------------------------------------
// FUNCIÓN AUXILIAR PARA LOS ORGANISMOS
// ---------------------------------------------------------

fn set_alive(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    framebuffer.set_current_color(alive_color());

    point(framebuffer, Vector2::new(x as f32, y as f32));
}

// ---------------------------------------------------------
// 1. BLOCK
// Still life.
// ---------------------------------------------------------

fn block(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(0, 0), (1, 0), (0, 1), (1, 1)];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 2. BEEHIVE
// Still life.
// ---------------------------------------------------------

fn beehive(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 3. LOAF
// Still life.
// ---------------------------------------------------------

fn loaf(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 4. BOAT
// Still life.
// ---------------------------------------------------------

fn boat(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 5. BLINKER
// Oscilador de período 2.
// ---------------------------------------------------------

fn blinker(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(0, 0), (1, 0), (2, 0)];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 6. TOAD
// Oscilador de período 2.
// ---------------------------------------------------------

fn toad(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 7. BEACON
// Oscilador de período 2.
// ---------------------------------------------------------

fn beacon(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 2),
        (2, 3),
        (3, 3),
    ];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 8. GLIDER
// Spaceship que viaja diagonalmente.
// ---------------------------------------------------------

fn glider(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 9. LIGHTWEIGHT SPACESHIP
// Spaceship que se desplaza horizontalmente.
// ---------------------------------------------------------

fn lwss(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [
        (1, 0),
        (4, 0),
        (0, 1),
        (0, 2),
        (4, 2),
        (0, 3),
        (1, 3),
        (2, 3),
        (3, 3),
    ];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 10. PULSAR
// Oscilador de período 3.
// ---------------------------------------------------------

fn pulsar(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [
        // Arriba
        (2, 0),
        (3, 0),
        (4, 0),
        (8, 0),
        (9, 0),
        (10, 0),
        (0, 2),
        (5, 2),
        (7, 2),
        (12, 2),
        (0, 3),
        (5, 3),
        (7, 3),
        (12, 3),
        (0, 4),
        (5, 4),
        (7, 4),
        (12, 4),
        (2, 5),
        (3, 5),
        (4, 5),
        (8, 5),
        (9, 5),
        (10, 5),
        // Abajo
        (2, 7),
        (3, 7),
        (4, 7),
        (8, 7),
        (9, 7),
        (10, 7),
        (0, 8),
        (5, 8),
        (7, 8),
        (12, 8),
        (0, 9),
        (5, 9),
        (7, 9),
        (12, 9),
        (0, 10),
        (5, 10),
        (7, 10),
        (12, 10),
        (2, 12),
        (3, 12),
        (4, 12),
        (8, 12),
        (9, 12),
        (10, 12),
    ];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// 11. GOSPER GLIDER GUN
//
// Esta estructura genera nuevos gliders periódicamente.
// ---------------------------------------------------------

fn gosper_glider_gun(framebuffer: &mut Framebuffer, x: i32, y: i32) {
    let cells = [
        // Bloque izquierdo
        (0, 4),
        (0, 5),
        (1, 4),
        (1, 5),
        // Parte central izquierda
        (10, 4),
        (10, 5),
        (10, 6),
        (11, 3),
        (11, 7),
        (12, 2),
        (12, 8),
        (13, 2),
        (13, 8),
        (14, 5),
        (15, 3),
        (15, 7),
        (16, 4),
        (16, 5),
        (16, 6),
        (17, 5),
        // Parte central derecha
        (20, 2),
        (20, 3),
        (20, 4),
        (21, 2),
        (21, 3),
        (21, 4),
        (22, 1),
        (22, 5),
        (24, 0),
        (24, 1),
        (24, 5),
        (24, 6),
        // Bloque derecho
        (34, 2),
        (34, 3),
        (35, 2),
        (35, 3),
    ];

    for (dx, dy) in cells {
        set_alive(framebuffer, x + dx, y + dy);
    }
}

// ---------------------------------------------------------
// PATRÓN INICIAL
// ---------------------------------------------------------

fn initial_state(framebuffer: &mut Framebuffer) {
    // Gun en la parte superior izquierda.
    gosper_glider_gun(framebuffer, 3, 4);

    // Still lifes en la parte superior derecha.
    block(framebuffer, 49, 6);

    beehive(framebuffer, 61, 6);

    loaf(framebuffer, 75, 6);

    boat(framebuffer, 91, 7);

    // Osciladores en la parte media derecha.
    blinker(framebuffer, 48, 25);

    toad(framebuffer, 63, 24);

    beacon(framebuffer, 82, 23);

    // Pulsar en la parte inferior izquierda.
    // Lo dejamos bastante separado para que pueda
    // oscilar claramente durante la demostración.
    pulsar(framebuffer, 6, 60);

    // Gliders.
    glider(framebuffer, 30, 48);

    glider(framebuffer, 42, 68);

    glider(framebuffer, 28, 86);

    // Lightweight spaceships.
    lwss(framebuffer, 62, 48);

    lwss(framebuffer, 74, 70);

    // Algunos organismos adicionales para llenar
    // más espacio sin usar células aleatorias.
    beacon(framebuffer, 91, 43);

    beehive(framebuffer, 54, 88);

    loaf(framebuffer, 86, 87);
}

// ---------------------------------------------------------
// MAIN
// ---------------------------------------------------------

fn main() {
    let window_width = 800;
    let window_height = 800;

    // La simulación ocurre realmente en 100x100.
    //
    // Después el framebuffer se escala a la ventana
    // de 800x800 para poder ver mejor cada célula.
    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Conway's Game of Life")
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    framebuffer.set_background_color(dead_color());

    // El laboratorio indica que no debemos limpiar
    // el framebuffer en cada frame.
    //
    // Por eso solamente hacemos clear una vez,
    // antes de colocar el patrón inicial.
    framebuffer.clear();

    initial_state(&mut framebuffer);

    // Mostramos primero el patrón inicial.
    framebuffer.swap_buffers(&mut window, &raylib_thread);

    // Dejamos visible la generación inicial un momento.
    thread::sleep(Duration::from_millis(700));

    while !window.window_should_close() {
        // Calculamos la siguiente generación.
        render(&mut framebuffer);

        // Mostramos el nuevo framebuffer.
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // Delay entre generaciones para que la
        // animación se pueda apreciar mejor.
        thread::sleep(Duration::from_millis(150));
    }
}
