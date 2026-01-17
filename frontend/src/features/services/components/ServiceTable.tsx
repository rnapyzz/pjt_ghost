import { Box } from "lucide-react";
import type { Service } from "../types";

type Props = {
  services: Service[];
  onClick: (slug: string) => void;
};

export function ServiceTable({ services, onClick }: Props) {
  return (
    <div className="rounded-lg border border-slate-200 bg-white over-flow-hidden">
      <table className="w-full text-left text-sm">
        <thead className="bg-slate-50 text-slate-500">
          <tr>
            <th className="px-6 py-2 font-medium">Name</th>
            <th className="px-6 py-2 font-medium">Slug</th>
            <th className="px-6 py-2 font-medium">Created at</th>
            <th className="px-6 py-2 font-medium">Updated at</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-slate-100">
          {services.map((s) => (
            <tr
              key={s.id}
              onClick={() => onClick(s.slug)}
              className="cursor-pointer hover:bg-slate-50 transition-colors text-slate-500"
            >
              <td className="px-6 py-3 flex hover:text-blue-500">
                <span className="w-3 h-3 mr-5 text-blue-600">
                  <Box />
                </span>
                {s.name}
              </td>
              <td className="px-6 py-3">{s.slug}</td>
              <td className="px-6 py-3 text-xs">
                {new Date(s.created_at).toLocaleDateString()}
              </td>
              <td className="px-6 py-3 text-xs">
                {new Date(s.updated_at).toLocaleDateString()}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
