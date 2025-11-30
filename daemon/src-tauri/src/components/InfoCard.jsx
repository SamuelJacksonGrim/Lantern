import React from 'react';

export default function InfoCard({ icon: Icon, title, desc, tech }) {
  return (
    <div className="p-8 bg-slate-900/50 backdrop-blur-sm border border-slate-800/50
      rounded-xl hover:border-orange-500/30 hover:bg-slate-900/70 transition-all group">
      <div className="flex items-start gap-4 mb-4">
        <div className="w-12 h-12 bg-gradient-to-br from-orange-400 to-amber-500
          rounded-lg flex items-center justify-center flex-shrink-0 group-hover:scale-110
          transition-transform shadow-lg shadow-orange-500/20">
          <Icon className="w-6 h-6 text-slate-950" />
        </div>
        <div>
          <h3 className="text-xl font-bold mb-1">{title}</h3>
          {tech && <div className="text-xs text-orange-400/60 code-block">{tech}</div>}
        </div>
      </div>
      <p className="text-slate-400 leading-relaxed">{desc}</p>
    </div>
  );
}
