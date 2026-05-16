// import { useState } from 'react';
export default function App() {
    // Temporary state placeholders
    // const [currentChord, setCurrentChord] = useState<string>("C Major");
    // const [selectedNotes, setSelectedNotes] = useState<string[]>(["C", "E", "G"]);


    return (
        <div className='min-h-screen bg-zinc-950 text-zinc-50 font-sans flex flex-col'>
            {/* Nav header flex*/}
            <header className="border-b border-zinc-800 bg-zinc-900/50 backdrop-blur-md px-6 py-4 flex justify-between items-center">
                <div className="flex items-center gap-3">
                    <span className="text-xl font-black tracking-tight text-amber-500">fretbored</span>
                    <span className="text-xs bg-zinc-800 px-2 py-0.5 rounded-full text-zinc-400 border border-zinc-700 font-mono">v0.1.0</span>
                </div>
                <nav className="text-sm font-medium text-zinc-400 flex gap-6">
                    <a href="#dashboard" className="text-zinc-100 hover:text-white transition">Dashboard</a>
                    <a href="#docs" className="hover:text-white transition">Documentation</a>
                </nav>
            </header>

            {/* Main Workspace Layout */}
            <main className="flex-1 max-w-7xl w-full mx-auto p-6 grid grid-cols-1 lg:grid-cols-4 gap-6">

            </main>

            {/* System Footer */}
            <footer className="border-t border-zinc-900 px-6 py-4 text-center text-xs text-zinc-600 font-mono">
                fretbored engine via WebAssembly & TypeScript
            </footer>
        </div>
    );

}

