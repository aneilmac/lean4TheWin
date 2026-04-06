use mdbook_preprocessor::book::{Book, Chapter};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};
use std::collections::HashMap;
use html_escape::encode_text_to_string;
use std::io;

const CONTEXT_PREFIX: &str = "--##";

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

struct LeanView;

impl Preprocessor for LeanView {
    fn name(&self) -> &str {
        "interactive-lean-view"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        book.for_each_chapter_mut(|ch| match process_chapter(ch) {
            Ok(s) => ch.content = s,
            Err(e) => eprintln!("failed to process chapter: {e:?}"),
        });
        Ok(book)
    }
}

fn process_chapter(ch: &Chapter) -> Result<String> {
    let parser = Parser::new(&ch.content);

    let mut events = Vec::new();

    let mut in_block = false;
    let mut code_lines: Vec<String> = Vec::new();
    let mut context: HashMap<usize, Vec<String>> = HashMap::new();
    let mut line_no = 1;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info)))
                if info.trim() == "lean-interactive" =>
            {
                in_block = true;
                code_lines.clear();
                context.clear();
                line_no = 1;
            }

            Event::End(TagEnd::CodeBlock) if in_block => {
                events.push(Event::HardBreak);
                events.push(Event::Html(render_block(&code_lines, &context).into()));
                in_block = false;
            }

            Event::Text(text) if in_block => {
                for line in text.lines() {
                    if let Some(idx) = line.find(CONTEXT_PREFIX) {
                        let msg = line[idx + CONTEXT_PREFIX.len()..].trim().to_string();
                        context.entry(line_no).or_default().push(msg);
                    } else {
                        code_lines.push(line.to_string());
                        line_no += 1;
                    }
                }
            }

            // swallow everything inside target block
            _ if in_block => {}
            // normal passthrough
            other => events.push(other),
        }
    }

    let mut out = String::new();
    pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut out)?;
    Ok(out)
}

fn render_block(code: &[String], context: &HashMap<usize, Vec<String>>) -> String {
    let mut escaped_code = String::new();
    encode_text_to_string(&code.join("\n"), &mut escaped_code, );

    let mut context_html = String::new();

    for (line, messages) in context {
        let mut escaped = String::new();
        encode_text_to_string(&messages.join("\n"), &mut escaped, );

        context_html.push_str(&format!(
            r#"<div class="lean-context-line" data-line="{}"><pre><samp>{}</samp></pre></div>"#,
            line, escaped
        ));
    }

    format!(
        r#"<div class="interactive-lean">
<div class="interactive-lean-code">
<pre><code class="language-lean hljs">{}</code></pre>
</div>
<div class="interactive-lean-context">
{}
</div>
</div>"#,
        escaped_code,
        context_html
    )
}

fn handle_preprocessing() -> Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;
    let processed = LeanView.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}