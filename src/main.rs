//! wordcount-rs: Klasik Unix `wc` komutunun Rust'la yazılmış, biraz
//! daha zengin bir versiyonu. Satır, kelime, karakter ve byte sayımının
//! yanında en sık kullanılan kelimeleri de gösterebilir.
//!
//! Kullanım:
//!     wordcount-rs dosya.txt
//!     wordcount-rs dosya1.txt dosya2.txt        # birden fazla dosya + toplam
//!     cat dosya.txt | wordcount-rs               # stdin'den okuma
//!     wordcount-rs dosya.txt --top 10             # en sık 10 kelimeyi göster
//!     wordcount-rs dosya.txt --json                # JSON çıktısı

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::process;

#[derive(Debug, Default, Clone)]
struct Counts {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

impl Counts {
    fn add(&mut self, other: &Counts) {
        self.lines += other.lines;
        self.words += other.words;
        self.chars += other.chars;
        self.bytes += other.bytes;
    }
}

fn count_text(text: &str) -> Counts {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();
    let bytes = text.len();

    Counts { lines, words, chars, bytes }
}

/// En sık kullanılan kelimeleri sayar. Kelimeler küçük harfe çevrilir
/// ve noktalama işaretlerinden ayıklanır, böylece "Merhaba," ve "merhaba"
/// aynı kelime olarak sayılır.
fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let cleaned: String = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        if !cleaned.is_empty() {
            *counts.entry(cleaned).or_insert(0) += 1;
        }
    }

    let mut pairs: Vec<(String, usize)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    pairs.truncate(n);
    pairs
}

fn read_input(path: Option<&str>) -> io::Result<String> {
    match path {
        Some(p) => fs::read_to_string(p),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn print_counts_table(label: &str, counts: &Counts) {
    println!(
        "{:<30} {:>10} {:>10} {:>10} {:>10}",
        label, counts.lines, counts.words, counts.chars, counts.bytes
    );
}

fn print_json(label: &str, counts: &Counts, words: &[(String, usize)]) {
    let words_json: Vec<String> = words
        .iter()
        .map(|(w, c)| format!("{{\"word\":\"{}\",\"count\":{}}}", escape_json(w), c))
        .collect();

    println!(
        "{{\"file\":\"{}\",\"lines\":{},\"words\":{},\"chars\":{},\"bytes\":{},\"top_words\":[{}]}}",
        escape_json(label),
        counts.lines,
        counts.words,
        counts.chars,
        counts.bytes,
        words_json.join(",")
    );
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

struct Args {
    files: Vec<String>,
    top: Option<usize>,
    json: bool,
}

fn parse_args(raw: &[String]) -> Args {
    let mut files = Vec::new();
    let mut top = None;
    let mut json = false;

    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--top" => {
                if let Some(val) = raw.get(i + 1) {
                    top = val.parse::<usize>().ok();
                    i += 1;
                }
            }
            "--json" => json = true,
            "--help" | "-h" => {
                print_help();
                process::exit(0);
            }
            other => files.push(other.to_string()),
        }
        i += 1;
    }

    Args { files, top, json }
}

fn print_help() {
    println!("wordcount-rs — satır/kelime/karakter sayım aracı\n");
    println!("Kullanım:");
    println!("  wordcount-rs <dosya...> [--top N] [--json]");
    println!("  cat dosya.txt | wordcount-rs [--top N] [--json]\n");
    println!("Seçenekler:");
    println!("  --top N    En sık kullanılan N kelimeyi göster");
    println!("  --json     Çıktıyı JSON formatında ver");
    println!("  --help     Bu yardım mesajını göster");
}

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    let args = parse_args(&raw_args);

    if args.files.is_empty() {
        // Stdin modu
        let text = match read_input(None) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Stdin okunamadı: {}", e);
                process::exit(1);
            }
        };

        let counts = count_text(&text);
        let words = if let Some(n) = args.top {
            top_words(&text, n)
        } else {
            Vec::new()
        };

        if args.json {
            print_json("stdin", &counts, &words);
        } else {
            println!(
                "{:<30} {:>10} {:>10} {:>10} {:>10}",
                "DOSYA", "SATIR", "KELİME", "KARAKTER", "BYTE"
            );
            print_counts_table("stdin", &counts);
            print_top_words_table(&words);
        }
        return;
    }

    // Dosya modu
    let mut total = Counts::default();
    let mut all_results: Vec<(String, Counts, String)> = Vec::new();
    let mut had_error = false;

    for file in &args.files {
        match read_input(Some(file)) {
            Ok(text) => {
                let counts = count_text(&text);
                total.add(&counts);
                all_results.push((file.clone(), counts, text));
            }
            Err(e) => {
                eprintln!("Hata: '{}' okunamadı: {}", file, e);
                had_error = true;
            }
        }
    }

    if !args.json {
        println!(
            "{:<30} {:>10} {:>10} {:>10} {:>10}",
            "DOSYA", "SATIR", "KELİME", "KARAKTER", "BYTE"
        );
    }

    for (file, counts, text) in &all_results {
        let words = if let Some(n) = args.top {
            top_words(text, n)
        } else {
            Vec::new()
        };

        if args.json {
            print_json(file, counts, &words);
        } else {
            print_counts_table(file, counts);
            print_top_words_table(&words);
        }
    }

    if all_results.len() > 1 && !args.json {
        print_counts_table("TOPLAM", &total);
    }

    if had_error {
        process::exit(1);
    }
}

fn print_top_words_table(words: &[(String, usize)]) {
    if words.is_empty() {
        return;
    }
    println!("  En sık kullanılan kelimeler:");
    for (word, count) in words {
        println!("    {:<20} {}", word, count);
    }
}
