#!/usr/bin/env python3
import argparse, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
EXPECTED=['rust-clean-coke-architecture-patterns','tdd-feature-workflow','rust-code-review','rust-ci-cd','rust-performance-optimization']
errors=[];warnings=[]
err=errors.append; warn=warnings.append

def parse_frontmatter(path):
    lines=path.read_text(encoding='utf-8',errors='ignore').splitlines()
    if not lines or lines[0] != '---': err(f'{path}: missing opening ---'); return
    end=None
    for i in range(1,len(lines)):
        if lines[i]=='---': end=i; break
    if end is None: err(f'{path}: missing closing ---'); return
    fm=lines[1:end]
    name=next((l.split(':',1)[1].strip() for l in fm if l.startswith('name:')), '')
    desc=next((l.split(':',1)[1].strip() for l in fm if l.startswith('description:')), '')
    if not name: err(f'{path}: missing name')
    if not desc: err(f'{path}: missing description')
    if end+1>=len(lines) or lines[end+1].strip()!='': err(f'{path}: markdown body must start after blank line')

def check_md(path):
    t=path.read_text(encoding='utf-8',errors='ignore'); lines=t.splitlines()
    if not lines: return
    avg=sum(map(len,lines))/len(lines)
    if avg>180: warn(f'{path}: suspicious average line length {avg:.1f}')
    for i,l in enumerate(lines,1):
        if len(l)>500 and not l.strip().startswith(('http','```')): warn(f'{path}:{i}: line >500 chars')
        if l.count('# ')>1: warn(f'{path}:{i}: multiple headings on one line')
        if l.count('- [ ]')>1: warn(f'{path}:{i}: multiple checklist markers on one line')
    if len(lines)<5 and (t.count('# ')>2 or t.count('- [ ]')>2): warn(f'{path}: likely collapsed/minified')

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--strict',action='store_true'); a=ap.parse_args()
    for s in EXPECTED:
        d=ROOT/s
        if not d.is_dir(): err(f'missing skill folder: {d}')
        for sub in ['workflows','references','templates']:
            if not (d/sub).is_dir(): err(f'{d}: missing {sub}/')
        sk=d/'SKILL.md'
        if not sk.exists(): err(f'{d}: missing SKILL.md')
        else: parse_frontmatter(sk)
        if not (d/'EVALS.md').exists(): err(f'{d}: missing EVALS.md')
    if not (ROOT/'README.md').exists(): err('missing backend/README.md')
    if not (ROOT/'templates/AGENTS.md').exists(): err('missing backend/templates/AGENTS.md')
    for md in ROOT.rglob('*.md'): check_md(md)
    ymls=[*ROOT.rglob('*.yml'),*ROOT.rglob('*.yaml')]
    try:
        import yaml
        for y in ymls:
            try: yaml.safe_load(y.read_text())
            except Exception as e: err(f'{y}: YAML parse error: {e}')
    except Exception:
        warn('PyYAML unavailable; full YAML parsing skipped')
        for y in ymls:
            if 'github-actions-' not in y.name:
                continue
            txt=y.read_text()
            for k in ['name:','on:','jobs:']:
                if k not in txt: warn(f'{y}: missing {k} (basic check)')
    print('skills checked:', ', '.join(EXPECTED))
    print('errors:',len(errors)); [print(' -',e) for e in errors]
    print('warnings:',len(warnings)); [print(' -',w) for w in warnings]
    fail=bool(errors) or (a.strict and bool(warnings))
    print('result:','FAIL' if fail else 'PASS')
    sys.exit(1 if fail else 0)
if __name__=='__main__': main()
