use igra::text::{EstiloTexto, MensagemFormatada, TipoMensagem};

fn main() {
    let tipos = [
        TipoMensagem::Sucesso,
        TipoMensagem::Aviso,
        TipoMensagem::Erro,
        TipoMensagem::Default,
        TipoMensagem::Info,
    ];
    let estilos = [
        EstiloTexto::Normal,
        EstiloTexto::Italico,
        EstiloTexto::Negrito,
        EstiloTexto::Sublinhado,
    ];
    for &tipo in &tipos {
        for &estilo in &estilos {
            let mensagem = MensagemFormatada::new(
                tipo,
                estilo,
                "Mensagem de Teste",
            );
            let formated = mensagem.formatar();
            println!("{}", formated);
        }
    }
}
