pub mod text {
    /// Enum EstiloTexto -> Estilos de formataçao
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum EstiloTexto {
        Normal,
        Italico,
        Negrito,
        Sublinhado,
    }

    /// Enum TipoMensagem -> Tipos de mensagens
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum TipoMensagem {
        Sucesso,
        Aviso,
        Erro,
        Info,
        Default,
    }

    use ansi_term::{Colour, Style};

    #[derive(Debug, Clone)]
    pub struct MensagemFormatada {
        tipo: TipoMensagem,
        estilo: EstiloTexto,
        texto: String,
    }

    impl MensagemFormatada {
        //Construtor novo
        pub fn new(tipo: TipoMensagem, estilo: EstiloTexto, texto: &str) -> Self {
            MensagemFormatada {
                tipo,
                estilo,
                texto: texto.to_string(),
            }
        }

        //Metodo para aplicar a formataçao
        pub fn formatar(&self) -> String {
            //Definir cor baseado no tipo
            let cor_base = match self.tipo {
                TipoMensagem::Sucesso => Colour::Green,
                TipoMensagem::Aviso => Colour::Yellow,
                TipoMensagem::Info => Colour::Blue,
                TipoMensagem::Erro => Colour::Red,
                TipoMensagem::Default => Colour::White,
            };
            let estilo_completo = match self.estilo {
                EstiloTexto::Normal => Style::new().fg(cor_base),
                EstiloTexto::Sublinhado => Style::new().underline().fg(cor_base),
                EstiloTexto::Negrito => Style::new().bold().fg(cor_base),
                EstiloTexto::Italico => Style::new().italic().fg(cor_base),
            };

            estilo_completo.paint(&self.texto).to_string()
        }

        //Metodo para imprimir
        pub fn printf(&self) {
            println!("{}", self.formatar());
        }
    }
}
