import { addMonths, format, parseISO, startOfMonth } from "date-fns";

// 指定した開始月から12ヶ月分の「yyyy-MM」のリストを生成する
// ex. ["2026-04", "2026-05"]
export const generateMonthColumns = (startDateStr: string = "2026-04-01") => {
  const start = startOfMonth(parseISO(startDateStr));
  return Array.from({ length: 12 }).map((_, i) => {
    const date = addMonths(start, i);
    return {
      key: format(date, "yyyy-MM-dd"),
      label: format(date, "M月"),
      fullLabel: format(date, "yyyy年M月"),
    };
  });
};

// 金額フォーマッター
export const formatCurrency = (amount: number | undefined) => {
  if (amount === undefined || amount === null) return "";
  return amount.toLocaleString();
};
