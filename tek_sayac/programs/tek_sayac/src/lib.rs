use anchor_lang::prelude::*;

declare_id!("DDzTgmzB7rehx8DtS88B2k4uf6VNxf9zJt7Mea8FUnHG");

// Programımızı tanımlıyoruz
#[program]
pub mod tek_sayac {
    use super::*;

    // İlk başlatma fonksiyonumuz - sayacı oluşturur ve sıfırdan başlatır
    pub fn baslat(ctx: Context<Baslat>) -> Result<()> {
        // Mesaj yazdırıyoruz
        msg!("Sayaç başlatılıyor...");
        
        // Sayaç hesabını alıyoruz
        let sayac = &mut ctx.accounts.sayac;
        
        // Sayacı sıfırlıyoruz
        sayac.sayi = 0;
        
        // Başlangıç değerini ekrana yazdırıyoruz
        msg!("Sayaç sıfırdan başlatıldı!");
        
        Ok(())
    }

    // Arttırma fonksiyonumuz - sayıyı bir arttırır
    pub fn artir(ctx: Context<Artir>) -> Result<()> {
        // Mesaj yazdırıyoruz
        msg!("Sayı arttırılıyor...");
        
        // Sayaç hesabını alıyoruz
        let sayac = &mut ctx.accounts.sayac;
        
        // Mevcut değeri yazdırıyoruz
        msg!("Eski değer: {}", sayac.sayi);
        
        // Sayıyı arttırıyoruz
        sayac.sayi += 1;
        
        // Yeni değeri yazdırıyoruz
        msg!("Yeni değer: {}", sayac.sayi);
        
        Ok(())
    }
}

// Başlatma işlemi için gerekli hesapları tanımlıyoruz
#[derive(Accounts)]
pub struct Baslat<'info> {
    // Yeni sayaç hesabı oluşturuyoruz
    #[account(
        init,               // Yeni hesap oluştur
        payer = kullanici,  // Ücreti kullanıcı ödeyecek
        space = 8 + 8      // Toplam alan: discriminator + sayı değeri
    )]
    pub sayac: Account<'info, SayacVeri>,
    
    // Kullanıcı hesabı
    #[account(mut)]
    pub kullanici: Signer<'info>,
    
    // Sistem programı
    pub system_program: Program<'info, System>,
}

// Arttırma işlemi için gerekli hesapları tanımlıyoruz
#[derive(Accounts)]
pub struct Artir<'info> {
    // Var olan sayaç hesabını kullanıyoruz
    #[account(mut)]
    pub sayac: Account<'info, SayacVeri>,
}

// Sayaç verilerimizin yapısı
#[account]
pub struct SayacVeri {
    pub sayi: u64,  // Sayımızı burada tutuyoruz
}