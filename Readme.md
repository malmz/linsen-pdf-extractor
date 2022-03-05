# Linsen pdf extractor

Reads the pdf menu from chalmers linsen resturant and outputs it in a nice structured format. 

## Usage

```ts
import { extract_pdf } from "https://deno.land/x/linsen-pdf-extractor/mod.ts";

const data = await Deno.readFile("./assets/meny.pdf");
const menu = extract_pdf(data);
```