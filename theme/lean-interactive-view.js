document.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll(".interactive-lean").forEach(container => {
    const pre = container.querySelector("pre");
    const code = container.querySelector("code");
    const context = container.querySelector(".interactive-lean-context");

    // Split code into lines
    const lines = code.innerHTML.split('\n');

    // Wrap each line in a span
    code.innerHTML = lines.map((line, i) => 
      `<span class="interactive-line" data-line="${i + 1}">${line}</span>`
    ).join('\n');

    // Add click handlers
    code.querySelectorAll(".interactive-line").forEach(lineSpan => {
      const contextLine = context.querySelector(".lean-context-line[data-line='" + lineSpan.dataset.line + "']");
      lineSpan.addEventListener("click", () => {
        code.querySelectorAll(".interactive-line").forEach(l => l.classList.remove("active"));
        lineSpan.classList.add("active");

        context.querySelectorAll(".lean-context-line").forEach(c => c.classList.remove("active"));
        contextLine?.classList.add("active");
      });
    });
  });
});