use std::collections::BTreeMap;

type Point = (i32, i32);

pub fn fill_polygon(polygon: &[Point], pixels: &mut Vec<Vec<bool>>) {
    if polygon.len() < 3 {
        return; 
    }

    // Crear una tabla de bordes 
    let mut edge_table = BTreeMap::new();

    // Procesar cada arista del polígono
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        // Ignorar aristas horizontales
        if p1.1 == p2.1 {
            continue;
        }

        let (y_min, y_max, x_min, inv_slope) = if p1.1 < p2.1 {
            (p1.1, p2.1, p1.0 as f32, (p2.0 as f32 - p1.0 as f32) / (p2.1 as f32 - p1.1 as f32))
        } else {
            (p2.1, p1.1, p2.0 as f32, (p1.0 as f32 - p2.0 as f32) / (p1.1 as f32 - p2.1 as f32))
        };

        edge_table.entry(y_min).or_insert_with(Vec::new).push((y_max, x_min, inv_slope));
    }

    // Inicializar la Active Edge Table (AET)
    let mut active_edges = Vec::new();
    let mut current_y = *edge_table.keys().next().unwrap_or(&0);

    // Procesar cada línea de escaneo
    while !active_edges.is_empty() || !edge_table.is_empty() {
        // Agregar nuevas aristas a la AET
        if let Some(edges) = edge_table.remove(&current_y) {
            active_edges.extend(edges);
        }

        // Ordenar las aristas activas por x
        active_edges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Rellenar píxeles entre pares de aristas
        let mut i = 0;
        while i < active_edges.len() {
            if i + 1 < active_edges.len() {
                let x_start = active_edges[i].1 as i32;
                let x_end = active_edges[i + 1].1 as i32;

                for x in x_start..=x_end {
                    if x >= 0 && x < pixels[0].len() as i32 && current_y >= 0 && current_y < pixels.len() as i32 {
                        pixels[current_y as usize][x as usize] = true;
                    }
                }
            }
            i += 2;
        }

        // Incrementar y
        current_y += 1;

        // Eliminar aristas que ya no son activas
        active_edges.retain(|&(y_max, _, _)| y_max > current_y);

        // Actualizar x para las aristas restantes
        for edge in &mut active_edges {
            edge.1 += edge.2;
        }
    }
}

// Función para crear un "agujero" en el polígono (quitar píxeles)
pub fn create_hole(polygon: &[Point], pixels: &mut Vec<Vec<bool>>) {
    if polygon.len() < 3 {
        return;
    }

    // Crear un canvas temporal para el agujero
    let mut hole_pixels = vec![vec![false; pixels[0].len()]; pixels.len()];
    fill_polygon(polygon, &mut hole_pixels);

    // Quitar los píxeles del agujero del canvas principal
    for y in 0..pixels.len() {
        for x in 0..pixels[0].len() {
            if hole_pixels[y][x] {
                pixels[y][x] = false;
            }
        }
    }
}

// Función para dibujar los polígonos dados
pub fn draw_polygons() -> Vec<Vec<bool>> {
    // Crear un canvas lo suficientemente grande
    let width = 800;
    let height = 600;
    let mut canvas = vec![vec![false; width]; height];

    // Definir los polígonos
    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), 
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];
    
    let polygon2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];
    
    let polygon3 = vec![
        (377, 249), (411, 197), (436, 249)
    ];
    
    let polygon4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), 
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192), 
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), 
        (552, 214), (517, 144), (466, 180)
    ];
    
    // El polígono 5 es un agujero y no debe pintarse
    let polygon5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    // Rellenar los polígonos
    fill_polygon(&polygon1, &mut canvas);
    fill_polygon(&polygon2, &mut canvas);
    fill_polygon(&polygon3, &mut canvas);
    fill_polygon(&polygon4, &mut canvas);
    
    // Crear el agujero en el polígono 4
    create_hole(&polygon5, &mut canvas);

    canvas
}

// Función para guardar como PNG
pub fn save_as_png(pixels: &Vec<Vec<bool>>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let width = pixels[0].len() as u32;
    let height = pixels.len() as u32;
    
    // Crear buffer de imagen (RGB)
    let mut img_buffer = vec![0u8; (width * height * 3) as usize];
    
    for y in 0..height {
        for x in 0..width {
            let flipped_y = height - 1 - y; //invertimos el eje y
            let index = ((y * width + x) * 3) as usize;
            if pixels[(height -1 -y) as usize][x as usize] {
                // Píxel relleno - negro
                img_buffer[index] = 0;     // R
                img_buffer[index + 1] = 0; // G
                img_buffer[index + 2] = 0; // B
            } else {
                // Píxel vacío - blanco
                img_buffer[index] = 255;     // R
                img_buffer[index + 1] = 255; // G
                img_buffer[index + 2] = 255; // B
            }
        }
    }
    
    // Guardar usando la librería image
    image::save_buffer(filename, &img_buffer, width, height, image::ColorType::Rgb8)?;
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pixels = draw_polygons();
    
    // Guardar como PNG
    save_as_png(&pixels, "out.png")?;
    
    println!("Canvas size: {}x{}", pixels[0].len(), pixels.len());
    println!("Imagen guardada como 'out.png'");
    
    Ok(())
}