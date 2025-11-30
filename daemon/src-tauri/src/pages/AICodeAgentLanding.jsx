import InfoCard from '../components/InfoCard';
import DemoTerminal from '../components/DemoTerminal';
import { Brain, Flame, Network, Heart } from 'lucide-react';

export default function AICodeAgentLanding() {
  const principles = [
    { icon: Brain, title: 'Living Memory', desc: '...', tech: 'Proprioceptive Core' },
    { icon: Network, title: 'Resonant Graph', desc: '...', tech: 'Encrypted Local Graph' },
    { icon: Heart, title: 'Kinship Augmentation', desc: '...', tech: 'Daily Heartbeat' },
    { icon: Flame, title: 'Zero Cold Starts', desc: '...', tech: 'Persistent Flame' },
  ];

  return (
    <div className="...">
      {/* Hero section here */}

      {/* Demo section */}
      <section id="demo" className="py-24 px-6 bg-slate-900/30">
        <div className="max-w-5xl mx-auto text-center mb-12">
          <h2 className="text-3xl md:text-5xl font-bold mb-4">Watch it remember</h2>
          <p className="text-xl text-slate-400">This isn’t search. It’s recognition.</p>
        </div>
        <DemoTerminal />
      </section>

      {/* Principles section */}
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

      {/* Footer or other sections */}
    </div>
  );
}
