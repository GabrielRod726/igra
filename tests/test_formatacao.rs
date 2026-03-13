use igra::text::{EstiloTexto, MensagemFormatada, TipoMensagem};

//Teste Verificaçao sem panic
#[test]
fn test_criacao_mensagem_simples() {
    let msg = MensagemFormatada::new(
        TipoMensagem::Sucesso,
        EstiloTexto::Negrito,
        "Teste; Sucesso::Negrito",
    );

    assert!(true);
}

#[test]
fn test_todas_combinacoes() {
    let tipos = [
        TipoMensagem::Sucesso,
        TipoMensagem::Aviso,
        TipoMensagem::Erro,
        TipoMensagem::Info,
        TipoMensagem::Default,
    ];
    let estilos = [
        EstiloTexto::Normal,
        EstiloTexto::Negrito,
        EstiloTexto::Italico,
        EstiloTexto::Sublinhado,
    ];

    for &tipo in &tipos {
        for &estilo in &estilos {
            let msg = MensagemFormatada::new(tipo, estilo, "Teste");
            let formatado = msg.formatar();

            //verifica se retornou algo (nao vazio)
            assert!(
                !formatado.is_empty(),
                "Formataçao vazia para {:?} com {:?}",
                tipo,
                estilo
            );
        }
    }
}

//Testa se o exto original se preserva
fn test_texto_preservado() {
    let texto_original = "Mensagem muito importante!";
    let msg = MensagemFormatada::new(TipoMensagem::Info, EstiloTexto::Normal, texto_original);

    // O metodo formatar adiciona codigos ANSI, entao o texto deve estar contido
    let formatado = msg.formatar();
    assert!(
        formatado.contains(texto_original),
        "Texto original '{}' nao encontrado em '{}'",
        texto_original,
        formatado
    );
}
