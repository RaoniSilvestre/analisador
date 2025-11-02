pub struct Analisador<'a> {
    pos: usize,
    prox: &'a str,
}

impl<'a> Analisador<'a> {
    pub fn novo(entrada: &'a str) -> Self {
        Self {
            pos: 0,
            prox: entrada,
        }
    }

    pub fn próximo(&mut self) -> Result<(usize, &str), Option<usize>> {
        let mut iter = self.prox.char_indices().peekable();
        let mut char_pos = self.pos;

        while let Some(&(inicio, c)) = iter.peek() {
            match c {
                ' ' | '🦀' => {
                    iter.next();
                    char_pos += 1;
                }

                c if c.is_ascii_digit() => {
                    let start_chart = char_pos + 1;

                    let resto = &self.prox[inicio..];
                    let fim = resto
                        .char_indices()
                        .find(|&(_, ch)| !ch.is_ascii_digit())
                        .map_or(resto.len(), |i| i.0);

                    let conteudo = &resto[..fim];
                    let restante = &resto[fim..];

                    self.prox = restante;

                    return Ok((start_chart, conteudo));
                }

                '🐧' | '+' | '-' | '/' | '*' => {
                    let start_char = char_pos + 1;
                    let len_bytes = c.len_utf8();

                    let fim = inicio + len_bytes;

                    let conteudo = &self.prox[inicio..fim];
                    let restante = &self.prox[fim..];

                    self.prox = restante;

                    return Ok((start_char, conteudo));
                }

                _ => {
                    return Err(Some(char_pos + 1));
                }
            }
        }

        Err(None)
    }
}
