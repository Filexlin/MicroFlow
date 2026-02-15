import { useState, useEffect } from 'react';
import { scanPythonLibrary, PythonScript } from '../services/api';

export function PythonLibraryPanel({ onSelect }: { onSelect: (s: PythonScript) => void }) {
  const [cats, setCats] = useState<Record<string, PythonScript[]>>({});
  
  useEffect(() => {
    scanPythonLibrary().then(setCats);
  }, []);
  
  return (
    <div>
      {Object.entries(cats).map(([cat, scripts]) => (
        <div key={cat}>
          <h4>{cat}</h4>
          {scripts.map(s => (
            <div key={s.relative_path} onClick={() => onSelect(s)}>
              <b>{s.name}</b>: {s.description}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}
