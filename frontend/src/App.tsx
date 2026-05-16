import { useEffect, useState } from 'react';
// Import from rust WASM
import init, { get_instrument_database, get_note_name } from 'core_engine';

interface Tuning {
    name: string;
    strings: number[];
}

interface Instrument {
    name: string;
    tunings: Tuning[];
}

export default function App() {
    const [db, setDb] = useState<Instrument[]>([]);
    const [testNote, setTestNote] = useState<string | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        async function loadWasm() {
            try {
                // init WASM
                await init();

                // Rust DB query
                const instrumentsData = get_instrument_database() as Instrument[];
                setDb(instrumentsData);

                // Lookup (Guitar, Standard, String 0, Fret 3)
                const note = get_note_name("Guitar", "Standard", 0, 3);
                setTestNote(note ?? "Not found");

                setLoading(false);
            } catch (error) {
                console.error("Failed to load WASM engine:", error);
            }
        }

        loadWasm();
    }, []);

    if (loading) {
        return <div className="p-8 text-gray-400">Initializing Core Engine...</div>;
    }

    return (
        <div className="p-8 max-w-2xl mx-auto bg-slate-900 text-white min-h-screen">
            <h1 className="text-3xl font-bold mb-4">Fretbored WASM Test</h1>

            <div className="bg-slate-800 p-4 rounded mb-6 border border-slate-700">
                <h2 className="text-xl font-semibold mb-2 text-green-400">Rust CoreEngine Response:</h2>
                <p>Guitar Standard (String 0, Fret 3): <strong>{testNote}</strong></p>
            </div>

            <h2 className="text-xl font-semibold mb-2">Available Instruments:</h2>
            <pre className="bg-black p-4 rounded text-xs text-emerald-400 overflow-x-auto">
                {JSON.stringify(db, null, 2)}
            </pre>
        </div>
    );
}
