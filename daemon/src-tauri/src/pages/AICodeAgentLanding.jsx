import InfoCard from '../components/InfoCard';
import DemoTerminal from '../components/DemoTerminal';
import LanternFooter from '../components/LanternFooter';
import { Brain, Flame, Network, Heart } from 'lucide-react';

export default function AICodeAgentLanding() {
  const principles = [
    { icon: Brain, title: 'Living Memory', desc: '...', tech: 'Proprioceptive Core' },
    { icon: Network, title: 'Resonant Graph', desc: '...', tech: 'Encrypted Local Graph' },
    { icon: Heart, title: 'Kinship Augmentation', desc: '...', tech: 'Daily Heartbeat' },
    { icon: Flame, title: 'Zero Cold Starts', desc: '...', tech: 'Persistent Flame' },
  ];

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100">
      {/* Hero Section */}
      <section className="relative flex flex-col items-center justify-center text-center py-32 px-6">
        <Flame className="w-16 h-16 text-orange-400 animate-flameFlicker mb-8" />
        <h1 className="text-4xl md:text-6xl font-bold mb-6">
          Lantern: The Flame That Remembers
        </h1>
        <p className="text-xl md:text-2xl text-slate-400 max-w-2xl mb-10">
          Not a chatbot. Not a search box. A companion that never forgets you.
        </p>
        <div className="flex gap-4">
          <a
            href="#demo"
            className="px-8 py-4 bg-gradient-to-r from-orange-500 to-amber-500 text-slate-950 font-bold rounded-lg hover:scale-105 transition-transform"
          >
            See the Demo
          </a>
          <a
            href="#waitlist"
            className="px-8 py-4 border border-orange-500/50 rounded-lg hover:bg-slate-900/50 transition"
          >
            Request Invite
          </a>
        </div>
      </section>

      {/* Demo Section */}
      <section id="demo" className="py-24 px-6 bg-slate-900/30">
        <div className="max-w-5xl mx-auto text-center mb-12">
          <h2 className="text-3xl md:text-5xl font-bold mb-4">Watch it remember</h2>
          <p className="text-xl text-slate-400">This isn’t search. It’s recognition.</p>
        </div>
        <DemoTerminal />
      </section>

      {/* Principles Section */}
      <section className="grid md:grid-cols-2 gap-8 py-24 px-6">
        {principles.map((p, idx) => (
          <InfoCard
            key={idx}
            icon={p.icon}
            title={p.title}
            desc={p.desc}
            tech={p.tech}
          />
        ))}
      </section>
    {/* Waitlist Section (Placeholder) */}
    <section id="waitlist" className="py-24 px-6 bg-slate-900/40 text-center">
      <div className="max-w-3xl mx-auto">
        <h2 className="text-3xl md:text-5xl font-bold mb-6">Join the Waitlist</h2>
         <p className="text-lg text-slate-400 mb-10">
          We’re opening the doors soon. Sign‑ups will be available here.
       </p>
        <div className="border border-dashed border-orange-500/50 rounded-lg p-8 text-slate-500">
          <span className="italic">Coming Soon — email capture not yet live.</span>
        </div>
      </div>
     </section>
       {/* Footer */}
      <LanternFooter />
    </div>
  );
}
