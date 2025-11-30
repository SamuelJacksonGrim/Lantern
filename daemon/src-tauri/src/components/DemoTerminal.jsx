import React from 'react';
import { Flame, Check } from 'lucide-react';

export default function DemoTerminal() {
  return (
    <div className="bg-slate-900/70 backdrop-blur-sm rounded-xl border border-slate-800/50 overflow-hidden shadow-2xl shadow-slate-950/50">
      <div className="bg-slate-800/70 px-4 py-3 flex items-center gap-2 border-b border-slate-700/50">
        <span className="text-slate-400 text-sm code-block">terminal</span>
        <div className="ml-auto flex items-center gap-2">
          <Flame className="w-4 h-4 text-orange-400 opacity-60 animate-flameFlicker" />
          <span className="text-xs text-slate-500">180ms wake time</span>
        </div>
      </div>
      <div className="p-6 code-block text-sm space-y-2.5">
        <div className="text-slate-300">
          <span className="text-orange-400">-&gt;</span> "user auth endpoints like the blog thing from May"
        </div>
        <div className="text-orange-400/60 pl-4 italic text-sm">remembering May 17th, 3:47 a.m., lo-fi playing...</div>
        <div className="text-slate-500 pl-4">/ Same Express middleware pattern you prefer</div>
        <div className="text-slate-500 pl-4">/ Zod validation (you hate try/catch soup)</div>
        <div className="text-emerald-400 pl-4 flex items-center gap-2 mt-3">
          <Check className="w-4 h-4" />
          <span>8 files. In your voice. Before you finished typing</span>
        </div>
      </div>
    </div>
  );
}
