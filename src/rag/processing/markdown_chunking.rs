use once_cell::sync::Lazy;
use regex::Regex;

use crate::rag::loading::loaded_data::LoadedFile;
use crate::rag::models::{chunks::Chunk, ChunkedFile, ImageRef};
use crate::rag::processing::ChunkSize;



pub fn split_markdown(file: LoadedFile, chunk_size: &ChunkSize) -> ChunkedFile<Chunk> {
    let chunk_size = *chunk_size as usize;
    let target_words = chunk_size.max(1);

    // 1) parse markdown into paragraph blocks + section path
    let blocks = extract_para_blocks(&file.content);
    if blocks.is_empty() {
        let chunks = vec![Chunk {
            seq_num: 0,
            text: file.content.clone(),
            embedding_vector: None,
            images: images_for_chunk(&file.content, &file.images),
        }];
        println!("EARLY CUNKS 2: {:#?}", chunks);
        return (file, chunks).into();
    }

    // 2) accumulate paragraphs until we reach the first size >= target_words.
    //    never cross a heading boundary for the same chunk.
    let mut chunks_text: Vec<String> = Vec::new();

    let mut cur_section = blocks[0].section_path.clone();
    let mut cur_body = String::new();
    let mut cur_words = 0usize;

    for (i, pb) in blocks.clone().into_iter().enumerate() {
        let heading_changed = !same_section_stack(&cur_section, &pb.section_path);

        if heading_changed && cur_words > 0 {
            chunks_text.push(make_chunk_text(&cur_section, &cur_body));
            cur_body.clear();
            cur_words = 0;
            cur_section = pb.section_path.clone();
        }

        if !cur_body.is_empty() {
            cur_body.push_str("\n\n"); // paragraph separator
        }
        cur_body.push_str(pb.body.trim_end());
        cur_words += pb.words;

        if cur_words >= target_words {
            chunks_text.push(make_chunk_text(&cur_section, &cur_body));
            cur_body.clear();
            cur_words = 0;

            if let Some(next_sec) = blocks.get(i + 1).map(|b| b.section_path.clone()) {
                cur_section = next_sec;
            }
        }
    }

    if cur_words > 0 {
        chunks_text.push(make_chunk_text(&cur_section, &cur_body));
    }

    // 3) merge tiny tails
    let tiny = (target_words / 3).max(1);
    let mut merged: Vec<String> = Vec::with_capacity(chunks_text.len());

    for ch in chunks_text.into_iter() {
        let wc = count_words(&ch);
        if let Some(last) = merged.last_mut() {
            let last_wc = count_words(last);
            if wc < tiny || last_wc < tiny {
                // merge small chunk into previous one, strip duplicate heading prefix from new part
                if !last.ends_with('\n') {
                    last.push('\n');
                }
                last.push('\n');
                last.push_str(strip_heading_prefix(&ch).trim());
                continue;
            }
        }
        merged.push(ch);
    }

    // 4) map to your Chunk with seq_num and empty embedding
    let chunks: Vec<Chunk> = merged
        .into_iter()
        .enumerate()
        .map(|(i, text)| Chunk {
            images: images_for_chunk(&text, &file.images),
            seq_num: i as i32,
            text,
            embedding_vector: None,
        })
        .collect();

    println!("CUNKS: {:#?}", chunks.len());

    (file, chunks).into()
}

fn images_for_chunk(text: &str, images: &[ImageRef]) -> Vec<ImageRef> {
    images
        .iter()
        .filter(|image| text.contains(&image.route))
        .cloned()
        .collect()
}

/* ---------------- internal helpers ---------------- */

#[derive(Debug, Clone)]
struct ParaBlock {
    section_path: Vec<(u8, String)>, // (level 1..6, title)
    body: String,                    // paragraph or fenced code block
    words: usize,
}

static WORD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[\p{Alphabetic}\p{Number}][\p{Alphabetic}\p{Number}\p{M}’'\-]*\b").unwrap()
});

fn count_words(s: &str) -> usize {
    WORD_RE.find_iter(s).count()
}

static ATX_HEADING_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<hash>#{1,6})\s+(?P<title>.+?)\s*#*\s*$").unwrap());
static FENCE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*```").unwrap());
static SETEXT_EQ_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*={3,}\s*$").unwrap());   // H1
static SETEXT_DASH_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*-{3,}\s*$").unwrap()); // H2

fn extract_para_blocks(md: &str) -> Vec<ParaBlock> {
    let mut blocks = Vec::<ParaBlock>::new();
    let mut section: Vec<(u8, String)> = Vec::new();

    let mut lines = md.lines().peekable();
    let mut cur_para = String::new();

    let mut push_para = |blocks: &mut Vec<ParaBlock>, section: &[(u8, String)], buf: &mut String| {
        let body = buf.trim().to_string();
        if !body.is_empty() {
            let words = count_words(&body);
            blocks.push(ParaBlock { section_path: section.to_vec(), body, words });
        }
        buf.clear();
    };

    while let Some(line) = lines.next() {
        let trimmed = line.trim_end();

        // fenced code (``` ... ```)
        if FENCE_RE.is_match(trimmed) {
            if !cur_para.trim().is_empty() {
                push_para(&mut blocks, &section, &mut cur_para);
            }
            let mut buf = String::new();
            buf.push_str(trimmed);
            buf.push('\n');
            while let Some(next) = lines.next() {
                buf.push_str(next);
                buf.push('\n');
                if FENCE_RE.is_match(next.trim_end()) {
                    break;
                }
            }
            let words = count_words(&buf);
            blocks.push(ParaBlock { section_path: section.clone(), body: buf.trim_end().to_string(), words });
            continue;
        }

        // ATX heading
        if let Some(cap) = ATX_HEADING_RE.captures(trimmed) {
            if !cur_para.trim().is_empty() {
                push_para(&mut blocks, &section, &mut cur_para);
            }
            let level = cap.name("hash").unwrap().as_str().len() as u8;
            let title = cap.name("title").unwrap().as_str().trim().to_string();
            // pop deeper or equal levels
            while let Some((lvl, _)) = section.last() {
                if *lvl >= level { section.pop(); } else { break; }
            }
            section.push((level, title));
            continue;
        }

        // Setext heading (current line text + next underline === or ---)
        if let Some(&next) = lines.peek() {
            let ntrim = next.trim_end();
            if SETEXT_EQ_RE.is_match(ntrim) || SETEXT_DASH_RE.is_match(ntrim) {
                if !cur_para.trim().is_empty() {
                    push_para(&mut blocks, &section, &mut cur_para);
                }
                let level = if SETEXT_EQ_RE.is_match(ntrim) { 1 } else { 2 };
                while let Some((lvl, _)) = section.last() {
                    if *lvl >= level { section.pop(); } else { break; }
                }
                section.push((level, trimmed.trim().to_string()));
                let _ = lines.next(); // consume underline
                continue;
            }
        }

        // blank line ends a paragraph
        if trimmed.trim().is_empty() {
            if !cur_para.trim().is_empty() {
                push_para(&mut blocks, &section, &mut cur_para);
            }
            continue;
        }

        // accumulate paragraph
        if !cur_para.is_empty() {
            cur_para.push('\n');
        }
        cur_para.push_str(trimmed);
    }

    if !cur_para.trim().is_empty() {
        let body = cur_para.trim().to_string();
        let words = count_words(&body);
        blocks.push(ParaBlock { section_path: section, body, words });
    }

    blocks
}

fn same_section_stack(a: &[(u8, String)], b: &[(u8, String)]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).all(|((la, ta), (lb, tb))| la == lb && ta == tb)
}

fn render_heading_prefix(section: &[(u8, String)]) -> String {
    // render stack: "# h1\n## h2\n### h3"
    let mut out = String::new();
    for (lvl, t) in section {
        let hashes = "#".repeat((*lvl as usize).min(6));
        out.push_str(&format!("{hashes} {t}\n"));
    }
    out.trim_end().to_string()
}

fn make_chunk_text(section: &[(u8, String)], body: &str) -> String {
    let header = render_heading_prefix(section);
    if header.is_empty() {
        body.trim().to_string()
    } else {
        let mut s = String::new();
        s.push_str(&header);
        s.push_str("\n\n");
        s.push_str(body.trim());
        s
    }
}

// strip leading heading lines (for merges)
fn strip_heading_prefix(s: &str) -> &str {
    let mut offset = 0usize;
    let mut consumed_heading = false;
    // split_inclusive keeps the line terminator, so the offsets stay inside the string
    // for both LF and CRLF and always land on a character boundary.
    for line in s.split_inclusive('\n') {
        let text = line.trim();
        if is_atx_heading(text) {
            consumed_heading = true;
            offset += line.len();
            continue;
        }
        if consumed_heading && text.is_empty() {
            offset += line.len();
            continue;
        }
        break;
    }
    &s[offset..]
}

/// True for a markdown ATX heading: one to six '#' followed by whitespace or end of line.
/// A data line such as `#7P600S300` is not a heading and has to survive the merge.
fn is_atx_heading(line: &str) -> bool {
    let hashes = line.chars().take_while(|c| *c == '#').count();
    if !(1..=6).contains(&hashes) {
        return false;
    }
    match line[hashes..].chars().next() {
        Some(c) => c.is_whitespace(),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::{split_markdown, strip_heading_prefix};
    use crate::rag::loading::loaded_data::LoadedFile;
    use crate::rag::RagProcessableFileType;

    fn markdown_file(content: String) -> LoadedFile {
        LoadedFile {
            file_type: RagProcessableFileType::Markdown,
            content,
            original_file_description: None,
            syntetic_file_description: None,
            internal_id: "test".to_string(),
            tags: None,
            images: vec![],
        }
    }

    // The string from the production panic. The last line has no trailing newline and
    // starts with '#', so the old offset math ran one byte past the end.
    #[test]
    fn keeps_content_line_that_starts_with_hash() {
        let text = "## Listing 1: Descripción del código\n\n#7P600S300&lt;cr&gt;";
        assert_eq!(strip_heading_prefix(text), "#7P600S300&lt;cr&gt;");
    }

    #[test]
    fn strips_nested_headings_and_one_blank_line() {
        assert_eq!(strip_heading_prefix("# H1\n## H2\n\nBody"), "Body");
    }

    #[test]
    fn keeps_plain_text_and_hashtags() {
        assert_eq!(strip_heading_prefix("Body only"), "Body only");
        assert_eq!(strip_heading_prefix("#hashtag stays"), "#hashtag stays");
    }

    #[test]
    fn handles_crlf_without_skipping_bytes() {
        assert_eq!(strip_heading_prefix("# H1\r\n\r\nCuerpo ó"), "Cuerpo ó");
    }

    #[test]
    fn returns_empty_when_only_headings_are_present() {
        assert_eq!(strip_heading_prefix("# H1\n## H2"), "");
    }

    // Same input through the real call path: the tiny tail chunk is merged into the
    // previous one, which is what calls strip_heading_prefix in production.
    #[test]
    fn merges_tiny_tail_without_panicking() {
        let long_paragraph = vec!["word"; 260].join(" ");
        let content = format!("# Title\n\n{long_paragraph}\n\n## Listing 1: Descripción del código\n\n#7P600S300&lt;cr&gt;\n");
        let chunked = split_markdown(markdown_file(content), &250);
        assert!(
            chunked.chunks.iter().any(|chunk| chunk.text.contains("#7P600S300")),
            "the listing line was dropped from the merged chunk"
        );
    }
}
