pub struct SinopseMessages {
    pub title: &'static str,
    pub messages: Vec<&'static str>,
}

pub fn get_sinopse_messages() -> SinopseMessages {
        return SinopseMessages {
            title: "Sinopse",
            messages: vec![
                "Maeyra e Ceara são como o Sol e a Lua, distantes, irmãs e… mágicas. Em um mundo onde magia é Energia Vital e nada pode quebrar um encantamento de sangue, as irmãs Campoalto precisam colocar feridas antigas e novos segredos para trás se quiserem proteger a única coisa em todo Kosmo que ainda as une, a caçula Ayla.",
                "Enquanto se preparam para lidar com as consequências desastrosas do Exame de iniciação de Ayla, as irmãs precisam navegar nas Águas cada vez mais turbulentas do Instituto de Magia de Anthor, o reino humano no continente bruxo. Maeyra, sempre astuta e controlada, secretamente entra numa Caçada perigosa com seus amigos bruxos pela Pedra-da-Lua — uma gema lendária capaz de mudar o curso da magia. Já Ceara repete todo mísero dia de Lua os mesmos pedidos à Deusa: desfazer seu vínculo com os Espíritos, cortar a cara torta de Darius em pedacinhos da próxima vez que ele der aquele sorriso cheio de dentes, continuar fingindo que não se importa com um certo humano e seu chaveirinho e garantir que as irmãs vão continuar vivas por mais uma Roda do ano. Quase nessa ordem.",
                "A Dança dos Encantos apresenta as contradições de uma sociedade mágica milenar, o conflito de corações, bruxos e humanos, em erupção e um lema: toda magia cobra um preço – e algumas escolhas podem custar mais que a própria vida.",
            ],
        };
}
