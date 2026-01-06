import { type Item } from "@/types";
import { generateMonthColumns, formatCurrency } from "@/lib/dateUtils";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"; // shadcn/uiのテーブルを使用

type Props = {
  items: Item[];
};

export const BudgetGrid = ({ items }: Props) => {
  // 今回は固定で「2026年4月始まり」にしていますが、将来的にはJobの開始日に合わせます
  const months = generateMonthColumns("2026-04-01");

  return (
    <div className="border rounded-md overflow-x-auto">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-50 min-w-50 bg-muted/50 sticky left-0 z-10">
              項目名
            </TableHead>
            {months.map((month) => (
              <TableHead key={month.key} className="text-right min-w-25">
                {month.label}
              </TableHead>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
          {items.map((item) => (
            <TableRow key={item.id}>
              {/* 固定列: 項目名 */}
              <TableCell className="font-medium bg-background sticky left-0 z-10 border-r">
                {item.name}
                {item.description && (
                  <div className="text-xs text-muted-foreground truncate max-w-45">
                    {item.description}
                  </div>
                )}
              </TableCell>

              {/* 変動列: 各月の金額 */}
              {months.map((month) => {
                // Item.entriesの中から、この月(month.key)に一致するEntryを探す
                // ※ APIのdateは "YYYY-MM-DD" なので完全一致で検索
                const entry = item.entries.find((e) => e.date === month.key);

                return (
                  <TableCell key={month.key} className="text-right">
                    {/* データがあれば金額を表示、なければハイフンや空文字 */}
                    {entry ? formatCurrency(entry.amount) : "-"}
                  </TableCell>
                );
              })}
            </TableRow>
          ))}

          {items.length === 0 && (
            <TableRow>
              <TableCell
                colSpan={months.length + 1}
                className="h-24 text-center text-muted-foreground"
              >
                データがありません。右上のボタンから項目を追加してください。
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </div>
  );
};
