#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationEntry {
    pub msgid: String,
    pub msgstr: String,
}

#[derive(Debug)]
pub struct TranslationCatalog {
    pub locale: String,
    entries: Vec<TranslationEntry>,
}

impl TranslationCatalog {
    pub fn new(locale: &str) -> Self {
        Self { locale: locale.to_string(), entries: Vec::new() }
    }
    pub fn add_message(&mut self, msgid: &str, msgstr: &str) {
        self.entries.push(TranslationEntry { msgid: msgid.into(), msgstr: msgstr.into() });
    }
    pub fn translate<'a>(&'a self, msgid: &'a str) -> &'a str {
        self.entries.iter().find(|e| e.msgid == msgid).map(|e| e.msgstr.as_str()).unwrap_or(msgid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_translation_catalog_aaa() {
        // Arrange
        let mut cat = TranslationCatalog::new("es_ES");
        cat.add_message("Cancel", "Cancelar");
        cat.add_message("Apply", "Aplicar");
        // Act
        let t1 = cat.translate("Cancel");
        let t2 = cat.translate("Help");
        // Assert
        assert_eq!(t1, "Cancelar");
        assert_eq!(t2, "Help");
    }
}
