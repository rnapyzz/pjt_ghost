export function ProjectPlanPage() {
  return (
    <div className="container mx-auto p-4 max-w-6xl">
      {/* header */}
      <div className="flex justify-between items-center mb-8 border-b pb-4">
        <div>
          <h1 className="text-xl font-semibold text-slate-700">
            Project Planning Sheet
          </h1>
          <p className="text-xs text-slate-600">
            プロジェクトの戦略と実行計画を策定します
          </p>
        </div>
        <div className="flex gap-2">
          <button className="px-4 py-1 text-gray-600 border rounded hover:bg-gray-50 text-sm">
            Back
          </button>
          <button className="px-4 py-1 bg-blue-600 border text-white rounded hover:bg-blue-700 text-sm">
            Save
          </button>
        </div>
      </div>

      {/* main */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {/* A */}
        <div className="md:col-span-1 space-y-6">
          <section className="bg-white p-5 rounded-lg border shadow-sm">
            <h2 className="text-lg font-semibold mb-4 text-gray-700">
              基本定義
            </h2>
            <div className="space-y-4">
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Project Name
              </label>
              <input className="w-full p-1.5 border rounded focus:ring-1 focus:outline-none focus:ring-blue-500" />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Type
              </label>
              <select className="w-full p-1.5 border rounded focus:ring-1 focus:outline-none focus:ring-blue-500">
                <option value=""></option>
              </select>
              <p className="text-xs text-gray-400 mt-1">
                ※ Agile: 柔軟な変更を前提 / Normal: 計画重視
              </p>
            </div>
          </section>
        </div>

        {/* B */}
        <div className="md:col-span-2 space-y-6">
          {/* value & target */}
          <section className="bg-white p-5 rounded-lg border shadow-sm">
            <h2 className="text-lg font-semibold mb-4 text-gray-700 items-center gap-2">
              Value & Customer
            </h2>
            <div className="grid grid-cols-1 gap-4">
              {/* value proposition */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Value Proposition
                </label>
                <textarea
                  rows={3}
                  placeholder="顧客にどのような価値や変革をもたらすか"
                  className="w-full p-2 border rounded text-sm focus:ring-1 focus:outline-none focus:ring-blue-500"
                />
              </div>

              {/* target */}
              <div className="grid grid-cols-2 gap-4">
                {/* market */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Target Market
                  </label>
                  <textarea
                    rows={3}
                    placeholder="どの市場・どの領域を狙うか"
                    className="w-full p-2 border rounded text-sm focus:ring-1 focus:outline-none focus:ring-blue-500"
                  />
                </div>
                {/* client */}
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Target Client
                  </label>
                  <textarea
                    rows={3}
                    placeholder="具体的な顧客像は"
                    className="w-full p-2 border rounded text-sm focus:ring-1 focus:outline-none focus:ring-blue-500"
                  />
                </div>
              </div>
            </div>
          </section>
          {/* kpi */}
          <section className="bg-white p-5 rounded-lg border shadow-sm">
            <h2 className="text-lg font-semibold mb-4 text-gray-700 flex items-center gap-2">
              KPIs
            </h2>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                重要業績評価指標
              </label>
              <textarea
                rows={3}
                placeholder="売上意外に追うべき指標（例：提案量、歩留率、単価、獲得数 etc...）"
                className="w-full p-2 border rounded text-sm focus:ring-1 focus:outline-none focus:ring-blue-500"
              />
            </div>
          </section>
          {/* pnl */}
          <section className="bg-gray-50 p-5 rounded-lg border border-dashed border-gray-300 text-center text-gray-400 text-sm">
            (P/L comming soon...)
          </section>
        </div>
      </div>
    </div>
  );
}
