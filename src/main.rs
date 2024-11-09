/*
 * Rust Resim Ödevi kodları
 * Şerafettin Doruk SEZER 
 * sdoruksezer@gmail.com
 * Githup: https://github.com/101PHOENIX
 */

 use macroquad::prelude::*;

 // yazı_yaz fonksiyonu
 async fn yazı_yaz() {
     draw_text("Selamunaleyküm Dünya", 180.0, 50.0, 30.0, WHITE);
 }
 
 // hilal_ciz fonksiyonu
 async fn hilal_ciz() {
     let x = 250.0;
     let y = 150.0;
     let radius = 75.0;
 
     // Hilal için iki çember çiz
     draw_circle(x, y, radius, WHITE); // Büyük beyaz çember
     draw_circle(x + 30.0, y, radius, BLACK); // Siyah çember 
 }
 
 // Parametresiz daire_cizgi fonksiyonu
 async fn daire_cizgi() {
     // Sabit değerler
     let çember_yarıçapı = 20.0;
     let çizgilerin_uzunlukları = 80.0;
 
     let start_x = 100.0;
     let start_y = 320.0;
     let spacing = çizgilerin_uzunlukları + çember_yarıçapı * 2.0; // daireler arasındaki boşluklar.
 
     // İlk dört daireyi çiz
     let mut points = vec![]; // dairelerin merkez koordinatlarını saklamak için bir vektör.
     for i in 0..4 {
         let x = start_x + i as f32 * spacing;
         let y = start_y;
         points.push((x, y));
 
         // Daireyi çiz (sarı renk)
         draw_circle(x, y, çember_yarıçapı, YELLOW);
     }
 
     // Daireler arasına okları çiz (▷ şeklinde)
     for i in 0..3 {
         let x1 = points[i].0;
         let y1 = points[i].1;
         let x2 = points[i + 1].0;
         let y2 = points[i + 1].1;
 
         // Çizgi çizin
         draw_line(
             x1 + çember_yarıçapı,  // İlk daireyi sağdan
             y1,
             x2 - çember_yarıçapı,  // İkinci daireyi soldan
             y2,
             2.0,
             WHITE,
         );
 
         // Ok gövdesi (-▷)
         draw_line(
             x1 + çember_yarıçapı, // Okun başlangıç x koordinatı
             y1, // Y koordinatı
             x2 - çember_yarıçapı, // Okun bitiş x koordinatı, kısaltıldı
             y2, // Y koordinatı
             2.0, 
             WHITE, 
         );
 
         // Ok başı (▷)
         draw_triangle(
             vec2(x1 + çember_yarıçapı + spacing * 0.4 + 20.0, y1 - 5.0), // Üst nokta
             vec2(x1 + çember_yarıçapı + spacing * 0.4 + 20.0, y1 + 5.0), // Alt nokta
             vec2(x1 + çember_yarıçapı + spacing * 0.4 + 35.0, y1), // Ok başı noktası
             WHITE,
         );
     }
 
     // üst ve alt daire yerleşimi
     let mid_x = (points[2].0 + points[3].0) / 2.0;
     let mid_y_top = points[2].1 - spacing;
     let mid_y_bottom = points[3].1 + spacing; // Dairelerin arasındaki boşluk kadar, üst ve alt dairelerin konumları tanımlanır.
 
     // Üstteki daire
     draw_circle(mid_x, mid_y_top, çember_yarıçapı, YELLOW);
     // Alttaki daire
     draw_circle(mid_x, mid_y_bottom, çember_yarıçapı, YELLOW);
 
     // En sondaki dairenin en üst ve alt noktaları
     let üst_nokta_y = points[3].1 - çember_yarıçapı;  // En sondaki dairenin en üst noktası
     let alt_nokta_y = points[3].1 + çember_yarıçapı;  // En sondaki dairenin alt noktası
 
     // Üstteki daireyi En sondaki dairenin üst noktasını işaret eden ok
     draw_line(
         mid_x,                 // Ok başlangıç noktası (üstteki daire)
         mid_y_top,             
         points[3].0,           // Ok bitiş noktası (En sondaki dairenin üst kısmı)
         üst_nokta_y,           
         2.0,                   
         WHITE,                 
     );
     // Ok başını üst noktaya doğru yerleştir
     draw_triangle(
         vec2(points[3].0 - 5.0, üst_nokta_y - 5.0),  // Ok başı üst nokta
         vec2(points[3].0 + 5.0, üst_nokta_y - 5.0),  // Ok başı alt nokta
         vec2(points[3].0, üst_nokta_y + 15.0),       // Ok başı noktası
         WHITE,                         
     );
 
     // Alttaki daireyi En sondaki dairenin alt noktasını işaret eden ok
     draw_line(
         mid_x,                 // Ok başlangıç noktası (alttaki daire)
         mid_y_bottom,          
         points[3].0,           // Ok bitiş noktası (En sondaki dairenin alt kısmı)
         alt_nokta_y,           
         2.0,                   
         WHITE,                 
     );
     // Ok başını alt noktaya doğru yerleştir
     draw_triangle(
         vec2(points[3].0 - 5.0, alt_nokta_y + 5.0), // Ok başı üst nokta
         vec2(points[3].0 + 5.0, alt_nokta_y + 5.0), // Ok başı alt nokta
         vec2(points[3].0, alt_nokta_y - 15.0),      // Ok başı noktası
         WHITE,                          // Renk
     );
 }
 
 // main fonksiyonu
 #[macroquad::main("Selamunaleyküm Dünya")]
 async fn main() {
     loop {
         // Arka planı siyah yap
         clear_background(BLACK);
 
         // Fonksiyonları çağır
         yazı_yaz().await;
         hilal_ciz().await;
         daire_cizgi().await;
 
         // Çizimleri ekrana yansıt
         next_frame().await;
     }
 } 