pub struct CharacterTextSplitter {
    chunk_size: usize,
    chunk_overlap: usize,
    separator: String,
}

impl CharacterTextSplitter {
    pub fn new(chunk_size: usize, chunk_overlap: usize, separator: String) -> Self {
        Self {
            chunk_size,
            chunk_overlap,
            separator,
        }
    }

    fn merge_splits(&self, splits: Vec<&str>) -> Vec<String> {
        let separator_len = self.separator.len();
        let mut docs: Vec<String> = Vec::new();
        let mut current_doc: Vec<&str> = Vec::new();
        let mut total = 0;

        for d in splits {
            let len = d.len();
            if total
                + len
                + if current_doc.is_empty() {
                    0
                } else {
                    separator_len
                }
                > self.chunk_size
            {
                if total > self.chunk_size {
                    println!(
                        "Created a chunk of size {}, which is longer than the specified {}",
                        total, self.chunk_size
                    );
                }
                if !current_doc.is_empty() {
                    let doc = current_doc.join(&self.separator);
                    if !doc.is_empty() {
                        docs.push(doc);
                    }
                    while total > self.chunk_overlap
                        || (total
                            + len
                            + if current_doc.is_empty() {
                                0
                            } else {
                                separator_len
                            }
                            > self.chunk_size
                            && total > 0)
                    {
                        total -= current_doc[0].len()
                            + if current_doc.len() > 1 {
                                separator_len
                            } else {
                                0
                            };
                        current_doc.remove(0);
                    }
                }
            }
            current_doc.push(d);
            total += len
                + if current_doc.len() > 1 {
                    separator_len
                } else {
                    0
                };
        }

        let doc = current_doc.join(&self.separator);
        if !doc.is_empty() {
            docs.push(doc);
        }

        docs
    }

    pub fn split_text(&self, text: &str) -> Vec<String> {
        let splits: Vec<&str> = text.split(&self.separator).collect();
        self.merge_splits(splits)
    }
}
