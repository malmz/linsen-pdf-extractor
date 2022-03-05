import { extract_pdf as wasm_extract_pdf } from "./pkg/linsen_pdf_extractor.js";

export interface Dishes {
  swedish: string[];
  english: string[];
}

export interface Menu {
  date: Date;
  dishes: Dishes;
}

export interface WeekMenu {
  days: Menu[];
}

export function extract_pdf(pdf_bytes: Uint8Array): WeekMenu {
  const data = wasm_extract_pdf(pdf_bytes) as {
    days: { date: string; dishes: Dishes }[];
  };

  const newDays = data.days.map((day) => {
    return {
      date: new Date(day.date),
      dishes: day.dishes,
    } as Menu;
  });

  return {
    days: newDays,
  } as WeekMenu;
}
