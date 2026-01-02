use std::io::{self, Write};

struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    println!("--- RUST TASK MANAGER V2 (FULL CRUD) ---");

    loop {
        println!("\nMenu: [1] Tambah [2] Lihat [3] Selesai [4] Hapus [5] Keluar");
        print!("Pilih menu: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Gagal membaca input");
        
        match choice.trim() {
            "1" => {
                print!("Masukkan deskripsi tugas: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).expect("Gagal");
                
                let new_task = Task {
                    id: next_id,
                    description: desc.trim().to_string(),
                    completed: false,
                };
                tasks.push(new_task);
                println!("Berhasil ditambahkan dengan ID: {}", next_id);
                next_id += 1;
            }
            "2" => {
                println!("\n--- DAFTAR TUGAS ANDA ---");
                if tasks.is_empty() {
                    println!("Belum ada tugas.");
                } else {
                    for t in &tasks {
                        let status = if t.completed { "[✓]" } else { "[ ]" };
                        println!("{} ID: {} - {}", status, t.id, t.description);
                    }
                }
            }
            "3" => {
                print!("Masukkan ID tugas yang selesai: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Gagal");
                
                if let Ok(target_id) = id_input.trim().parse::<u32>() {
                    let mut found = false;
                    for t in &mut tasks {
                        if t.id == target_id {
                            t.completed = true;
                            found = true;
                            println!("Tugas ID {} ditandai selesai!", target_id);
                        }
                    }
                    if !found { println!("ID tidak ditemukan."); }
                }
            }
            "4" => {
                print!("Masukkan ID tugas yang ingin dihapus: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Gagal");

                if let Ok(target_id) = id_input.trim().parse::<u32>() {
                    let initial_len = tasks.len();
                    // Menghapus data dengan filter: simpan yang ID-nya TIDAK sama
                    tasks.retain(|t| t.id != target_id);
                    
                    if tasks.len() < initial_len {
                        println!("Tugas ID {} berhasil dihapus!", target_id);
                    } else {
                        println!("ID tidak ditemukan.");
                    }
                }
            }
            "5" => {
                println!("Keluar... Tetap semangat belajarnya, Madan!");
                break;
            }
            _ => println!("Pilihan tidak valid, coba lagi."),
        }
    }
}