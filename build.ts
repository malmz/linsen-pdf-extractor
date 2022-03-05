await Deno.run({
  cmd: ["cargo", "build", "--release", "--target=wasm32-unknown-unknown"],
}).status();

await Deno.run({
  cmd: [
    "wasm-bindgen",
    "--out-dir",
    "pkg",
    "--target",
    "deno",
    "./target/wasm32-unknown-unknown/release/linsen_pdf_extractor.wasm",
  ],
}).status();

await Deno.run({
  cmd: [
    "wasm-opt",
    "pkg/linsen_pdf_extractor_bg.wasm",
    "-o",
    "pkg/linsen_pdf_extractor_bg.wasm",
    "-Oz",
  ],
}).status();
