use std::sync::Arc;

use eframe::egui::{self, text::LayoutJob, Color32, FontId, TextFormat};
use tree_sitter::{Node, Parser};

const TEXT: Color32 = Color32::from_rgb(244, 239, 251);
const KEYWORD: Color32 = Color32::from_rgb(196, 167, 231);
const STRING: Color32 = Color32::from_rgb(235, 190, 145);
const COMMENT: Color32 = Color32::from_rgb(141, 117, 158);
const TYPE: Color32 = Color32::from_rgb(151, 202, 193);
const NUMBER: Color32 = Color32::from_rgb(224, 175, 255);

pub fn layout_job(
    ui: &egui::Ui,
    text: &str,
    wrap_width: f32,
    extension: &str,
) -> Arc<egui::Galley> {
    let mut job = LayoutJob::default();
    job.wrap.max_width = wrap_width;
    let font = FontId::monospace(ui.text_style_height(&egui::TextStyle::Monospace));
    let format = TextFormat {
        font_id: font.clone(),
        color: TEXT,
        ..Default::default()
    };

    let Some(language) = language_for(extension) else {
        job.append(text, 0.0, format);
        return ui.fonts(|fonts| fonts.layout_job(job));
    };

    let mut parser = Parser::new();
    if parser.set_language(&language).is_err() {
        job.append(text, 0.0, format);
        return ui.fonts(|fonts| fonts.layout_job(job));
    }
    let Some(tree) = parser.parse(text, None) else {
        job.append(text, 0.0, format);
        return ui.fonts(|fonts| fonts.layout_job(job));
    };

    let mut spans = Vec::new();
    collect_leaf_spans(tree.root_node(), text, &mut spans);
    spans.sort_by_key(|span| span.0);
    let mut cursor = 0;
    for (start, end, kind) in spans {
        if start < cursor || end > text.len() {
            continue;
        }
        if start > cursor {
            job.append(&text[cursor..start], 0.0, format.clone());
        }
        if end > start {
            let mut span_format = format.clone();
            span_format.color = color_for(kind);
            job.append(&text[start..end], 0.0, span_format);
            cursor = end;
        }
    }
    if cursor < text.len() {
        job.append(&text[cursor..], 0.0, format);
    }
    ui.fonts(|fonts| fonts.layout_job(job))
}

fn language_for(extension: &str) -> Option<tree_sitter::Language> {
    match extension {
        "rs" => Some(tree_sitter_rust::LANGUAGE.into()),
        _ => None,
    }
}

fn collect_leaf_spans(node: Node<'_>, text: &str, spans: &mut Vec<(usize, usize, &'static str)>) {
    if node.child_count() == 0 {
        let start = node.start_byte();
        let end = node.end_byte();
        if end <= text.len() && end > start {
            spans.push((start, end, node.kind()));
        }
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_leaf_spans(child, text, spans);
    }
}

fn color_for(kind: &str) -> Color32 {
    if kind.contains("comment") {
        COMMENT
    } else if kind.contains("string") || kind.contains("char") {
        STRING
    } else if kind.contains("integer") || kind.contains("float") {
        NUMBER
    } else if kind.ends_with("_type") || kind == "type_identifier" || kind == "primitive_type" {
        TYPE
    } else if matches!(
        kind,
        "fn" | "let"
            | "mut"
            | "struct"
            | "enum"
            | "impl"
            | "trait"
            | "use"
            | "mod"
            | "pub"
            | "const"
            | "static"
            | "match"
            | "if"
            | "else"
            | "for"
            | "while"
            | "return"
            | "self"
            | "true"
            | "false"
    ) {
        KEYWORD
    } else {
        TEXT
    }
}
