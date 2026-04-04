//! Splits a `lean-interactive` code section into a `lean` code section
//! And the context elements where applicable.

use mdbook_preprocessor::book::{Book, Chapter};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{Event, Parser, Tag, TagEnd, CodeBlockKind, CowStr};
use std::collections::HashMap;
use std::io;

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
        book.for_each_chapter_mut(|ch| match generate_lean_view(ch) {
            Ok(s) => ch.content = s,
            Err(e) => eprintln!("failed to process chapter: {e:?}"),
        });
        Ok(book)
    }
}

fn generate_lean_view(chapter: &mut Chapter) -> Result<String> {
    let parser = Parser::new(&chapter.content);

    let mut events = Vec::new();
    let mut line_count: usize = 1;
    let mut in_target_block = false;
    let mut code_buffer = String::new();
    let mut context_buffer = HashMap::<usize, CowStr>::new();
    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                if info.trim() == "lean-interactive"
                {
                    in_target_block = true;
                    code_buffer.clear();
                    context_buffer.clear();
                }
                else {
                    events.push(Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))));
                }
            }
            Event::End(TagEnd::CodeBlock) if in_target_block => {
                events.push(Event::HardBreak);
                events.push(Event::Html(CowStr::from(r#"<div class="interactive-lean">"#)));
                
                events.push(Event::Html(CowStr::from(r#"<div class="interactive-lean-code">"#)));
                events.push(Event::Html(CowStr::from("<pre>")));
                events.push(Event::Html(CowStr::from(r#"<code class="language-lean hljs">"#)));
                events.push(Event::Html(CowStr::from(code_buffer.clone())));
                events.push(Event::Html(CowStr::from("</code>")));
                events.push(Event::Html(CowStr::from("</pre>")));
                events.push(Event::Html(CowStr::from("</div>")));

                events.push(Event::Html(CowStr::from(r#"<div class="interactive-lean-context">"#)));
                for (line_num, context_line) in context_buffer.iter() {
                    events.push(Event::Html(CowStr::from(format!(
                        r#"<div class="lean-context-line" data-line="{}"><pre><code>{}</code></pre></div>"#,
                        line_num, context_line
                    ))));
                }

                events.push(Event::Html(CowStr::from(r#"</div>"#)));
                events.push(Event::Html(CowStr::from(r#"</div>"#)));
                in_target_block = false;

            }

            Event::Text(text) if in_target_block => {
                eprintln!("TEXT: {text}");
                if text.trim_start().starts_with("--##")
                {
                    if context_buffer.contains_key(&line_count)
                    {
                         context_buffer.insert(line_count, 
                            CowStr::from(context_buffer[&line_count].to_string() + &text));
                    } 
                    else 
                    {
                        context_buffer.insert(line_count, text);   
                    }
                }
                else 
                {
                    line_count += 1;
                    code_buffer.push_str(&text);                    
                }
            }
            other => {
                events.push(other);
            }
        }
    }

    let mut out = String::new();
    pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut out)?;

    Ok(out)
}

pub fn handle_preprocessing() -> Result<()> {
    let pre = LeanView;
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;
    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}