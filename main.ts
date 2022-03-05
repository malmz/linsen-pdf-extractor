import { extract_pdf } from "./mod.ts";

const data = await Deno.readFile("./linsen-pdf-extractor/assets/meny.pdf");
const menu = await extract_pdf(data);
console.log(menu);
