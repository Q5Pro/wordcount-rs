# 📊 wordcount-rs

Klasik Unix `wc` komutunun Rust ile yazılmış, biraz daha zengin bir
versiyonu. Satır, kelime, karakter ve byte sayımının yanında en sık
kullanılan kelimeleri de gösterebilir, JSON çıktısı verebilir.

## Özellikler

- 📏 Satır, kelime, karakter ve byte sayımı (UTF-8 doğru işlenir)
- 📚 Birden fazla dosya desteği + otomatik toplam satırı
- 🏆 `--top N` ile en sık kullanılan N kelimeyi gösterme (noktalama
  ayıklanır, büyük/küçük harf duyarsız)
- 📥 Dosyadan veya stdin'den (pipe) okuma
- 🔄 `--json` ile makine tarafından işlenebilir çıktı
- ⚡ Rust ile yazıldığı için hızlı, `release` modunda LTO optimizasyonu aktif

## Kurulum

```bash
cargo build --release
# Derlenen ikili dosya: target/release/wordcount-rs
```

## Kullanım

```bash
# Tek dosya
wordcount-rs metin.txt

# Birden fazla dosya (otomatik toplam satırı eklenir)
wordcount-rs bolum1.txt bolum2.txt bolum3.txt

# Stdin'den oku
cat metin.txt | wordcount-rs
curl -s https://example.com/metin.txt | wordcount-rs

# En sık kullanılan 10 kelimeyi göster
wordcount-rs metin.txt --top 10

# JSON çıktısı (script'lerde işlemek için)
wordcount-rs metin.txt --top 5 --json
```

| Parametre | Açıklama |
|---|---|
| `--top N` | En sık kullanılan N kelimeyi göster |
| `--json` | Çıktıyı JSON formatında ver |
| `--help`, `-h` | Yardım mesajını göster |

## Örnek çıktı

```
DOSYA                               SATIR     KELİME   KARAKTER       BYTE
makale.txt                             42        612       3104       3180
  En sık kullanılan kelimeler:
    ve                   38
    bir                  29
    bu                   24
```

## `wc` ile karşılaştırma

Satır, kelime ve byte sayıları sistemin `wc` komutuyla doğrulanmıştır:

```bash
$ wc -lwc dosya.txt
  3  21 138 dosya.txt

$ wordcount-rs dosya.txt
DOSYA                               SATIR     KELİME   KARAKTER       BYTE
dosya.txt                               3         21        135        138
```

(Karakter sayısı byte sayısından küçük olabilir — bu, UTF-8'de
çok-byte'lık karakterlerin, örneğin Türkçe "ı", "ş", "ğ" gibi
karakterlerin, birden fazla byte ama tek karakter olarak sayılmasından
kaynaklanır; bu doğru ve beklenen davranıştır.)

## Lisans

MIT


---

> Made in [discord.gg/codeshare](https://discord.gg/codeshare) · [astra-dev.com.tr](https://astra-dev.com.tr)
