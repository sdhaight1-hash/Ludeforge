// Original bounded balance research. Run: node research/prototypes/balance-check.mjs
function study(){
 const checks=[];const test=(n,ok)=>{if(!ok)throw Error(n);checks.push(n)};
 const items=[{id:"blade",cost:3,group:"weapon",attack:4,guard:0},{id:"staff",cost:2,group:"weapon",attack:2,guard:1},{id:"mail",cost:4,group:"armor",attack:0,guard:5},{id:"leather",cost:2,group:"armor",attack:0,guard:2},{id:"training",cost:2,attack:2,guard:1},{id:"mastery",cost:2,requires:"training",attack:3,guard:0},{id:"trinket",cost:1,attack:0,guard:0},{id:"shield",cost:3,attack:0,guard:4}];
 const budget=8;
 function feasible(xs){const ids=new Set(xs.map(x=>x.id)),g=xs.map(x=>x.group).filter(Boolean);return xs.reduce((s,x)=>s+x.cost,0)<=budget&&new Set(g).size===g.length&&xs.every(x=>!x.requires||ids.has(x.requires))}
 function score(xs){const ids=xs.map(x=>x.id).sort();return {id:ids.join("+")||"empty",cost:xs.reduce((s,x)=>s+x.cost,0),attack:xs.reduce((s,x)=>s+x.attack,0),guard:xs.reduce((s,x)=>s+x.guard,0)}}
 const exhaustive=[];for(let mask=0;mask<2**items.length;mask++){const xs=items.filter((_,i)=>mask&(1<<i));if(feasible(xs))exhaustive.push(score(xs))}
 const pruned=[];let nodes=0;function walk(i,xs,cost,groups){nodes++;if(i===items.length){if(feasible(xs))pruned.push(score(xs));return}walk(i+1,xs,cost,groups);const x=items[i];if(cost+x.cost<=budget&&(!x.group||!groups.has(x.group)))walk(i+1,[...xs,x],cost+x.cost,new Set([...groups,...(x.group?[x.group]:[])]))}
 walk(0,[],0,new Set());
 const sort=x=>x.map(v=>v.id).sort().join("|");
 test("pruned enumeration equals exhaustive legal builds",sort(exhaustive)===sort(pruned));
 const dominates=(a,b)=>a.attack>=b.attack&&a.guard>=b.guard&&a.cost<=b.cost&&(a.attack>b.attack||a.guard>b.guard||a.cost<b.cost);
 const front=exhaustive.filter(b=>!exhaustive.some(a=>dominates(a,b))).sort((a,b)=>a.cost-b.cost||a.id.localeCompare(b.id));
 test("every omitted build has a dominating legal witness",exhaustive.filter(x=>!front.includes(x)).every(b=>front.some(a=>dominates(a,b))));
 test("pareto front internally nondominated",front.every(b=>!front.some(a=>dominates(a,b))));
 test("prerequisite not silently granted",!exhaustive.some(x=>x.id==="mastery"));
 const best=(metric)=>exhaustive.reduce((a,b)=>b[metric]>a[metric]?b:a);
 test("attack and guard objectives differ",best("attack").id!==best("guard").id);
 // Exact factorial sensitivity for independent discrete binary factors; use conditional means.
 const rows=[];for(const a of [-1,1])for(const b of [-1,1])for(const c of [-1,1])rows.push({a,b,c,y:10+2*a+3*a*b});
 const variance=rows.reduce((s,r)=>s+(r.y-10)**2,0)/rows.length;
 const effects={};for(const k of ["a","b","c"]){const means=[-1,1].map(v=>rows.filter(r=>r[k]===v).reduce((s,r)=>s+r.y,0)/4);const first=means.reduce((s,m)=>s+(m-10)**2,0)/2/variance;
 const rest=["a","b","c"].filter(x=>x!==k);let conditional=0;for(const v of [-1,1])for(const w of [-1,1]){const pair=rows.filter(r=>r[rest[0]]===v&&r[rest[1]]===w);const m=(pair[0].y+pair[1].y)/2;conditional+=pair.reduce((s,r)=>s+(r.y-m)**2,0)/2}
 effects[k]={first,total:conditional/4/variance}}
 test("known variance",variance===13);test("interaction-only main effect hidden",effects.b.first===0&&effects.b.total===9/13);test("inactive factor",effects.c.first===0&&effects.c.total===0);test("combined factor total",effects.a.total===1&&effects.a.first===4/13);
 // finite two-player zero-sum matrix, exact rational numerator over common denominator.
 const rps=[[0,-1,1],[1,0,-1],[-1,1,0]];
 function regret(matrix,p,q){const den=p.reduce((s,x)=>s+x,0),denQ=q.reduce((s,x)=>s+x,0);const pay=matrix.map(row=>row.reduce((s,v,j)=>s+v*q[j],0));const column=q.map((_,j)=>matrix.reduce((s,row,i)=>s+row[j]*p[i],0));const actual=pay.reduce((s,v,i)=>s+v*p[i],0);return {rowGainNumerator:Math.max(...pay)*den-actual,columnGainNumerator:actual-Math.min(...column)*denQ,denominator:den*denQ}}
 const mixed=regret(rps,[1,1,1],[1,1,1]),pure=regret(rps,[1,0,0],[1,0,0]),biased=regret(rps,[3,1,0],[3,1,0]);
 test("uniform RPS no unilateral gain",mixed.rowGainNumerator===0&&mixed.columnGainNumerator===0);test("pure RPS exploitable",pure.rowGainNumerator===1&&pure.columnGainNumerator===1);test("biased RPS exploitable",biased.rowGainNumerator>0&&biased.columnGainNumerator>0);
 // Feedback: exact integer perturbations of discrete toy state, no fitted parameters.
 const trajectories={};for(const [name,fn] of [["damping",x=>Math.trunc(x/2)],["reinforcing",x=>2*x],["oscillation",x=>-x]]){const seq=[8];for(let i=0;i<6;i++)seq.push(fn(seq.at(-1)));trajectories[name]=seq}
 test("damping decays",trajectories.damping.at(-1)===0);test("reinforcing grows",trajectories.reinforcing.at(-1)===512);test("undamped oscillation",trajectories.oscillation.every(x=>Math.abs(x)===8));
 return {namedAssertions:checks.length,checks,builds:{candidateSubsets:2**items.length,legal:exhaustive.length,pareto:front.length,prunedSearchNodes:nodes,front,bestAttack:best("attack"),bestGuard:best("guard")},sensitivity:{rows,variance,effects},game:{matrix:rps,mixed,pure,biased},feedback:trajectories,limitations:"Original finite deterministic model. Build score proxies not encounter outcomes; exact front only in this fixture. Independent equiprobable binary parameters; no general Sobol estimator. Complete information finite zero-sum matrix only; no general TTRPG equilibrium. Feedback scalar integer recurrence, no realistic simulation. No performance, Rust, or human validity claims."}
}
console.log(JSON.stringify(study(),null,2));
