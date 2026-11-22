"""Original storage trade-off probe; not a product benchmark."""
import json
import os
import platform
import sqlite3
import statistics
import tempfile
import time
from pathlib import Path


def write_new(path, rows):
    with path.open('x', encoding='utf-8', newline='\n') as f:
        json.dump(rows, f, separators=(',', ':'), sort_keys=True)
        f.flush()
        os.fsync(f.fileno())


def measure(n, root):
    rows = [{'id': f'row{i}', 'rank': 2, 'role': 'r'} for i in range(n)]
    source = root / f'{n}.json'
    write_new(source, rows)
    original = source.read_bytes()
    db = sqlite3.connect(root / f'{n}.db')
    db.execute('PRAGMA journal_mode=DELETE')
    db.execute('PRAGMA synchronous=FULL')
    db.execute('CREATE TABLE item(id TEXT PRIMARY KEY, rank INTEGER NOT NULL, role TEXT NOT NULL)')
    db.executemany('INSERT INTO item VALUES (:id,:rank,:role)', rows)
    db.commit()
    target = f'row{n//2}'
    json_times, sql_times = [], []
    for i in range(6):  # one warm-up, five measured edits
        rank = 1 + (i % 3)
        start = time.perf_counter_ns()
        candidate = json.loads(source.read_bytes())
        next(row for row in candidate if row['id'] == target)['rank'] = rank
        out = root / f'{n}-{i}.json'
        write_new(out, candidate)
        json_us = (time.perf_counter_ns() - start) // 1000
        start = time.perf_counter_ns()
        with db:
            db.execute('UPDATE item SET rank=? WHERE id=?', (rank, target))
        sql_us = (time.perf_counter_ns() - start) // 1000
        # Verify equivalent data outside timers, including all unaffected rows.
        restored = [dict(zip(('id', 'rank', 'role'), r)) for r in db.execute('SELECT id,rank,role FROM item')]
        assert sorted(json.loads(out.read_bytes()), key=lambda r:r['id']) == sorted(restored, key=lambda r:r['id'])
        assert source.read_bytes() == original
        if i:
            json_times.append(json_us)
            sql_times.append(sql_us)
    before = db.execute('SELECT rank FROM item WHERE id=?', (target,)).fetchone()
    try:
        with db:
            db.execute('UPDATE item SET rank=99 WHERE id=?', (target,))
            raise RuntimeError('simulated application exception before commit')
    except RuntimeError:
        pass
    assert db.execute('SELECT rank FROM item WHERE id=?', (target,)).fetchone() == before
    db.close()
    broken = root / f'{n}-partial.json'
    with broken.open('xb') as f:
        f.write(original[:len(original)//2]); f.flush(); os.fsync(f.fileno())
    try:
        json.loads(broken.read_bytes())
    except (ValueError, UnicodeError):
        pass
    else:
        raise AssertionError('partial fixture parsed')
    assert source.read_bytes() == original
    return {'rows': n, 'source_bytes': len(original), 'json_samples_us': json_times, 'sqlite_samples_us': sql_times, 'json_median_us': statistics.median(json_times), 'sqlite_median_us': statistics.median(sql_times), 'equivalent_edits': 6, 'exception_rollback': True, 'partial_new_file_rejected': True, 'original_unchanged': True}


if __name__ == '__main__':
    with tempfile.TemporaryDirectory() as tmp:
        result = {'python': platform.python_version(), 'sqlite': sqlite3.sqlite_version, 'platform': platform.system(), 'results': [measure(n, Path(tmp)) for n in (100, 1000, 10000)], 'limits': 'Synthetic flat rows, one edit; JSON full read and synced new file vs indexed SQLite update with FULL sync and DELETE journal. No strict Ludeforge parser, semantic constraint validation, same durability guarantee, crash/power-loss simulation, concurrent writers or production performance.'}
    print(json.dumps(result, indent=2))
