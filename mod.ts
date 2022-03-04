import { CachePolicy, prepare } from "https://deno.land/x/plug@0.5.1/plug.ts";

function readPointer(v: Deno.UnsafePointer): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v);
  const lengthBe = new Uint8Array(4);
  const view = new DataView(lengthBe.buffer);
  ptr.copyInto(lengthBe, 0);
  const buf = new Uint8Array(view.getUint32(0));
  ptr.copyInto(buf, 4);
  return buf;
}

const opts = {
  name: "linsen-pdf-extractor",
  url: new URL("./target/debug", import.meta.url).toString(),
  policy: CachePolicy.NONE,
};

const library = await prepare(opts, {
  extract_pdf: {
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: true,
  },
});

export type Menu = {
  date: Date;
  dishes: string[];
};

export type WeekMenu = {
  days: Menu[];
};

type Return = {
  days: [
    {
      date: string;
      dishes: string[];
    }
  ];
};

export async function extract_pdf(pdf_data: Uint8Array): Promise<WeekMenu> {
  const res = await library.symbols.extract_pdf(pdf_data, pdf_data.byteLength);
  const data = readPointer(res);
  const text = new TextDecoder().decode(data);
  const json = JSON.parse(text) as Return;

  const menu: WeekMenu = {
    days: json.days.map((menu) => {
      return {
        date: new Date(menu.date),
        dishes: menu.dishes,
      };
    }),
  };

  return menu;
}
