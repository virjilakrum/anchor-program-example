import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TekSayac } from "../target/types/tek_sayac";

describe("tek_sayac", () => {
    // Test ortamını hazırlıyoruz
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.TekSayac as Program<TekSayac>;
    const sayacHesabi = anchor.web3.Keypair.generate();

    // İlk test: Sayacı başlatma
    it("Sayaç başlatılıyor!", async () => {
        await program.methods
            .baslat()
            .accounts({
                sayac: sayacHesabi.publicKey,
                kullanici: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([sayacHesabi])
            .rpc();

        const hesap = await program.account.sayacVeri.fetch(sayacHesabi.publicKey);
        console.log("Başlangıç değeri:", hesap.sayi.toString());
    });

    // İkinci test: Sayıyı arttırma
    it("Sayı arttırılıyor!", async () => {
        await program.methods
            .artir()
            .accounts({
                sayac: sayacHesabi.publicKey,
            })
            .rpc();

        const hesap = await program.account.sayacVeri.fetch(sayacHesabi.publicKey);
        console.log("Arttırıldıktan sonraki değer:", hesap.sayi.toString());
    });
});