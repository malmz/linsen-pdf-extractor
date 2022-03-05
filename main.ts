import { extract_pdf } from "./mod.ts";

const data = await Deno.readFile("./assets/meny.pdf");
console.time("extract_pdf");
const menu = extract_pdf(data);
console.timeEnd("extract_pdf");
console.log(menu.days[0].dishes);
